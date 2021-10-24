use std::sync::{MutexGuard, PoisonError};

use diesel::mysql;

pub enum AuthError<'a> {
    Diesel(diesel::result::Error),
    Mutex(MutexGuard<'a, mysql::MysqlConnection>),
    ErrInfo(String)
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

impl From<String> for AuthError<'_> {
    fn from(s: String) -> Self {
        Self::ErrInfo(s)
    }
}
