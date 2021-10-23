use crate::{
    forms::auth::{ChangePaswd, UserAuth},
    generate_controller,
    models::user::{NewUser, User},
};
use diesel::{mysql, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{
    http::{Cookie, CookieJar, Status},
    post,
    response::status,
    serde::json::Json,
    State,
};
use std::{sync::Mutex, time::Duration};

use self::{
    auth_key::AuthKey,
    utils::{check_login, check_password_size, password_hash},
};

mod auth_key;
mod from_request;
mod utils;

const COOKIE_NAME: &str = "__UIF";
const AUTH_LIFE_TIME: u64 = 3600;

generate_controller!(
    UserAuthCtrl,
    "/auth",
    user_auth,
    new_user,
    log_out,
    change_passwd
);

#[post("/login", data = "<input>")]
fn user_auth(
    input: Json<UserAuth>,
    db: &State<Mutex<mysql::MysqlConnection>>,
    cookies: &CookieJar<'_>,
) -> (Status, String) {
    if let Some(user) = check_login(cookies).and_then(|f| f.into_full_user(db).ok()) {
        return (
            Status::AlreadyReported,
            format!("you[{}] have logined", user.name),
        );
    }
    let hashed_pwd = password_hash(&input.paswd);

    let res = {
        use crate::models::schema::users::dsl::*;
        let db = db.lock().expect("Mutex Failure");
        users
            .filter(email.eq(&input.email))
            .filter(password.eq(&hashed_pwd))
            .first::<User>(&*db)
    };

    if let Ok(user) = res {
        let mut u = input.0.clone();
        u.id = Some(user.id);
        cookies.add_private(AuthKey::new_cookie(
            COOKIE_NAME,
            u,
            Duration::from_secs(AUTH_LIFE_TIME),
        ));

        (Status::Ok, format!("User[{}] login success", user.name))
    } else {
        (Status::Forbidden, "Wrong Password Or Email Address".into())
    }
}

#[post("/signup", data = "<data>")]
fn new_user(
    data: Json<NewUser>,
    db: &State<Mutex<mysql::MysqlConnection>>,
    cookies: &CookieJar<'_>,
) -> status::Accepted<String> {
    if let Some(user) = check_login(cookies).and_then(|f| f.into_full_user(db).ok()) {
        status::Accepted(Some(format!("You Has Been logined {}", user.name)))
    } else {
        if data.check_able(db) {
            use crate::models::schema::users::dsl::*;
            let db = &*db.lock().expect("Lock Mutex Error");
            let data = data.encode_password(password_hash);

            diesel::insert_into(users)
                .values(&data)
                .execute(db)
                .expect("Insert User Error");
            status::Accepted(Some(format!("New Account Create : name:{}", data.name)))
        } else {
            status::Accepted(Some(format!("Email can not be use [{}|<=128] Or Name[{}|<=32],Password[8=<|{}|<=64] Over Limit Size", 
            data.email,data.name,data.password)))
        }
    }
}

#[post("/logout")]
fn log_out(user_auth: UserAuth, cookies: &CookieJar<'_>) -> status::Accepted<String> {
    let mut ak = AuthKey::new(user_auth, Duration::ZERO);
    ak.kill();
    cookies.add_private(Cookie::new(
        COOKIE_NAME,
        serde_json::to_string(&ak).expect("Prase To Json Error"),
    ));
    status::Accepted(Some("Logout Success".into()))
}

#[post("/change-paswd", data = "<data>")]
fn change_passwd(
    data: Json<ChangePaswd>,
    user_auth: UserAuth,
    cookies: &CookieJar<'_>,
    db: &State<Mutex<mysql::MysqlConnection>>,
) -> Option<String> {
    let new_hash = password_hash(&data.new);
    if data.old == user_auth.paswd && data.new == data.new_conf && check_password_size(&data.new) {
        let user = user_auth.into_full_user(db).ok()?;

        let res = {
            use crate::models::schema::users::dsl::*;
            let db = db.lock().expect("Failure Lock");
            diesel::update(users)
                .filter(id.eq(user.id))
                .set(password.eq(&new_hash))
                .execute(&*db)
                .ok()?
        };
        // update cookies
        let mut ua = user_auth.clone();
        ua.paswd = (&data.new).into();
        
        let auth = AuthKey::new_cookie(COOKIE_NAME, ua, Duration::from_secs(AUTH_LIFE_TIME));
        cookies.add_private(auth);

        Some(format!("change success,{}", res))
    } else {
        None
    }
}
