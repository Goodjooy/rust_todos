use std::{
    fmt::Display,
    sync::{MutexGuard, PoisonError},
};

use diesel::mysql;

pub enum AuthError {
    Diesel(diesel::result::Error),
    ErrInfo(String),
}

impl From<diesel::result::Error> for AuthError {
    fn from(err: diesel::result::Error) -> Self {
        Self::Diesel(err)
    }
}

impl From<String> for AuthError {
    fn from(s: String) -> Self {
        Self::ErrInfo(s)
    }
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::Diesel(de) => de.fmt(f),
            AuthError::ErrInfo(s) => s.fmt(f),
        }
    }
}
