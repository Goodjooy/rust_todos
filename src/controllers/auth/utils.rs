use crypto::digest::Digest;
use diesel::{mysql, QueryDsl, RunQueryDsl};
use rocket::http::CookieJar;
use std::sync::{Mutex, MutexGuard, PoisonError};

use crate::{
    forms::auth::UserAuth,
    models::user::{NewUser, User},
};

use super::{auth_key::AuthKey, COOKIE_NAME};

const EMAIL_REGEX: &str = r#"^\\w+([-_.]?\\w+)*@\\w+([\\.-]?\\w+)*(\\.\\w{2,6})+$"#;

pub enum AuthError<'a> {
    Diesel(diesel::result::Error),
    Mutex(MutexGuard<'a, mysql::MysqlConnection>),
}

pub fn password_hash(password: &str) -> String {
    let mut hasher = crypto::sha3::Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}

pub fn check_login(cookies: &CookieJar<'_>, db: &Mutex<mysql::MysqlConnection>) -> Option<User> {
    let cookie = cookies.get_private(COOKIE_NAME)?;

    let auth_key = AuthKey::<UserAuth>::from_cookie(cookie, COOKIE_NAME, cookies)?;

    let res = {
        use crate::models::schema::users::dsl::*;
        use diesel::ExpressionMethods;
        let db = db.lock().expect("Mutex Failure");
        if let Some(aid) = auth_key.id {
            users.filter(id.eq(aid)).limit(1).load::<User>(&*db)
        } else {
            users
                .filter(email.eq(&auth_key.email))
                .filter(password.eq(&auth_key.paswd))
                .limit(1)
                .load::<User>(&*db)
        }
        .expect("Find User Failure")
    };
    if res.len() == 1 {
        res.into_iter().next()
    } else {
        None
    }
}

impl NewUser<'_> {
    pub fn check_able(&self, db: &Mutex<mysql::MysqlConnection>) -> bool {
        self.check_email_avaiable() && self.check_email_used(db)
    }

    fn check_email_used(&self, db: &Mutex<mysql::MysqlConnection>) -> bool {
        use crate::models::schema::users::dsl::*;

        let db = db.lock().unwrap();
        use diesel::ExpressionMethods;
        let res = users
            .filter(email.eq(&self.email))
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
    fn check_email_avaiable(&self) -> bool {
        let reg = regex::Regex::new(EMAIL_REGEX).unwrap();

        reg.is_match(self.email)
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
            users.filter(id.eq(aid)).limit(1).first::<User>(&*db)
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
