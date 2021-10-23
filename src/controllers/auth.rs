use crate::{
    forms::auth::UserAuth,
    models::user::{NewUser, User},
};
use diesel::{mysql, QueryDsl, RunQueryDsl};

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
    utils::{check_login, password_hash},
};

use crate::generate_controller;

mod auth_key;
mod from_request;
mod utils;

const COOKIE_NAME: &str = "__UIF";

generate_controller!(UserAuthCtrl, "/auth", user_auth, new_user, log_out);

#[post("/login", data = "<input>")]
fn user_auth(
    input: Json<UserAuth>,
    db: &State<Mutex<mysql::MysqlConnection>>,
    cookie: &CookieJar<'_>,
) -> (Status, String) {
    let hashed_pwd = password_hash(&input.paswd);

    let res = {
        use crate::models::schema::users::dsl::*;
        use diesel::ExpressionMethods;
        let db = db.lock().expect("Mutex Failure");
        users
            .filter(email.eq(&input.email))
            .filter(password.eq(&hashed_pwd))
            .first::<User>(&*db)
    };

    if let Ok(user) = res {
        let mut u = input.0.clone();
        u.id = Some(user.id);
        cookie.add_private(AuthKey::new_cookie(
            COOKIE_NAME,
            u,
            Duration::from_secs(30 * 60),
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
    if let Some(user) = check_login(cookies, db) {
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
            status::Accepted(Some(format!("Email can not be use [{}]", data.email)))
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
