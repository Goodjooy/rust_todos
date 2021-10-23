use crypto::digest::Digest;
use diesel::{mysql, QueryDsl, RunQueryDsl};
use rocket::http::CookieJar;
use std::sync::{Mutex, MutexGuard, PoisonError};

use crate::{
    forms::auth::UserAuth,
    models::user::{NewUser, User},
};

use super::{auth_key::AuthKey, COOKIE_NAME};

const EMAIL_REGEX: &str = r#"^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$"#;

pub enum AuthError<'a> {
    Diesel(diesel::result::Error),
    Mutex(MutexGuard<'a, mysql::MysqlConnection>),
}

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

impl NewUser<'_> {
    pub fn check_able(&self, db: &Mutex<mysql::MysqlConnection>) -> bool {
        check_email_avaiable(self.email)
            && check_email_used(self.email, db)
            && check_name_size(self.name)
            && check_password_size(&self.password)
    }
}
pub fn check_email_used(mail: &str, db: &Mutex<mysql::MysqlConnection>) -> bool {
    use crate::models::schema::users::dsl::*;

    let db = db.lock().unwrap();
    use diesel::ExpressionMethods;
    let res = users
        .filter(email.eq(mail))
        .count()
        .get_result::<i64>(&*db);

    if let Ok(num) = res {
        if num == 0 {
            true
        } else {
            false
        }
    } else {
        true
    }
}
pub fn check_email_avaiable(email: &str) -> bool {
    let reg = regex::Regex::new(EMAIL_REGEX).unwrap();

    reg.is_match(email) && email.len() <= 128
}

pub fn check_name_size(name: &str) -> bool {
    name.len() <= 32
}
pub fn check_password_size(passwd: &str) -> bool {
    match passwd.len() {
        s if (s < 8 || s > 64) => false,
        _ => true,
    }
}

impl UserAuth {
    pub fn get_id(&self) -> Option<u32> {
        let res = &self.id?;
        Some(*res)
    }

    pub fn into_full_user<'a>(
        &self,
        db: &'a Mutex<mysql::MysqlConnection>,
    ) -> Result<User, AuthError<'a>> {
        use crate::models::schema::users::dsl::*;
        use diesel::ExpressionMethods;

        let db = db.lock()?;
        let res = if let Some(aid) = self.id {
            users.filter(id.eq(aid)).first::<User>(&*db)
        } else {
            users
                .filter(email.eq(&self.email))
                .filter(password.eq(&self.paswd))
                .first::<User>(&*db)
        }?;

        Ok(res)
    }
}

impl<'a> From<PoisonError<MutexGuard<'a, mysql::MysqlConnection>>> for AuthError<'a> {
    fn from(src: PoisonError<MutexGuard<'a, mysql::MysqlConnection>>) -> Self {
        Self::Mutex(src.into_inner())
    }
}

impl From<diesel::result::Error> for AuthError<'_> {
    fn from(err: diesel::result::Error) -> Self {
        Self::Diesel(err)
    }
}
