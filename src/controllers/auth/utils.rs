use crypto::digest::Digest;

use rocket::http::CookieJar;

use crate::forms::auth::UserAuth;

use super::{auth_key::AuthKey, COOKIE_NAME};

pub use auth_err::*;
pub use new_user::*;
pub use user_auth::*;

mod auth_err;
mod new_user;
mod user_auth;

pub fn password_hash(password: &str) -> String {
    let mut hasher = crypto::sha3::Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}

pub fn check_login(cookies: &CookieJar<'_>) -> Option<UserAuth> {
    let cookie = cookies.get_private(COOKIE_NAME)?;

    let auth_key = AuthKey::<UserAuth>::from_cookie(cookie, COOKIE_NAME, cookies)?;

    Some(auth_key)
}
