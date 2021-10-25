use crate::{
    database::DatabaseConnection,
    forms::{
        auth::{ChangePaswd, UserAuth},
        RResult,
    },
    generate_controller,
    models::user::{NewUser, User},
    to_rresult, update_first_or_create,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{
    http::{Cookie, CookieJar},
    post,
    response::status,
    serde::json::Json,
    State,
};
use std::time::Duration;

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
    db: &State<DatabaseConnection>,
    cookies: &CookieJar<'_>,
) -> RResult<String> {
    if let Some(user) = check_login(cookies).and_then(|f| f.into_full_user(db).ok()) {
        return RResult::err(format!("you[{}] have logined", user.name));
    }
    let hashed_pwd = password_hash(&input.paswd);

    let res = {
        use crate::models::schema::users::dsl::*;
        let db = to_rresult!(rs, db.get());
        crate::load_first!(
            &db,
            User,
            users,
            email.eq(&input.email),
            password.eq(&hashed_pwd)
        )
    };
    let user = to_rresult!(rs, res, "Wrong Password Or Email Address");
    let mut u = input.0.clone();
    u.id = Some(user.id);
    cookies.add_private(AuthKey::new_cookie(
        COOKIE_NAME,
        u,
        Duration::from_secs(AUTH_LIFE_TIME),
    ));

    RResult::ok(format!("User[{}] login success", user.name))
}

#[post("/signup", data = "<data>")]
fn new_user(
    data: Json<NewUser>,
    db: &State<DatabaseConnection>,
    cookies: &CookieJar<'_>,
) -> RResult<String> {
    if let Some(user) = check_login(cookies).and_then(|f| f.into_full_user(db).ok()) {
        RResult::err(format!("You Has Been logined {}", user.name))
    } else {
        if data.check_able(db) {
            use crate::models::schema::users::dsl::*;
            let db = to_rresult!(rs, db.get());
            let data = data.encode_password(password_hash);

            let _ = to_rresult!(
                rs,
                crate::insert_into!(&db, users, &data),
                "Failure To Add New User"
            );

            RResult::ok(format!("New Account Create : name:{}", data.name))
        } else {
            RResult::err(format!("Email can not be use [{}|<=128] Or Name[{}|<=32],Password[8=<|{}|<=64] Over Limit Size", 
            data.email,data.name,data.password))
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
    db: &State<DatabaseConnection>,
) -> RResult<String> {
    let new_hash = password_hash(&data.new);
    if data.old == user_auth.paswd && data.new == data.new_conf && check_password_size(&data.new) {
        let user = to_rresult!(rs, user_auth.into_full_user(db));
        {
            use crate::models::schema::users::dsl::*;
            let db = db.get().expect("Failure Lock");
            let temp = NewUser::from_au_pc(&user_auth, &data);
            let t = update_first_or_create!(
                &db,
                User,
                users,
                temp,
                pk=> user.id,
                set => [password.eq(&new_hash)]
            );
            to_rresult!(rs, t);
        };
        // update cookies
        let mut ua = user_auth.clone();
        ua.paswd = (&data.new).into();

        let auth = AuthKey::new_cookie(COOKIE_NAME, ua, Duration::from_secs(AUTH_LIFE_TIME));
        cookies.add_private(auth);

        RResult::ok("change success".into())
    } else {
        RResult::err("New password cannot the same with new || password len over size")
    }
}
