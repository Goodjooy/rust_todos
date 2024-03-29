use rocket::outcome::Outcome;
use std::io::Cursor;

use rocket::{
    http::{ContentType, Status},
    response::Responder,
    Response,
};
use serde::Serialize;

pub mod auth;
pub mod user_detail;
pub mod todo;

#[derive(Serialize)]
pub struct RResult<T: Serialize> {
    err: bool,
    emsg: Option<String>,
    data: Option<T>,
}

impl<T: Serialize> RResult<T> {
    fn new(err: bool, emsg: Option<String>, data: Option<T>) -> Self {
        RResult { err, emsg, data }
    }

  

    pub fn from_result<E: ToString>(res: Result<T, E>) -> Self {
        match res {
            Ok(data) => Self::new(false, None, Some(data)),
            Err(err) => Self::new(true, Some(err.to_string()), None),
        }
    }
    pub fn from_option<E: ToString>(res: Option<T>, info: E) -> Self {
        Self::from_result(res.ok_or(info))
    }

    pub fn ok(data: T) -> Self {
        Self::new(false, None, Some(data))
    }
    pub fn err<I: ToString>(msg: I) -> Self {
        Self::new(true, Some(msg.to_string()), None)
    }
    pub fn into_outcome(self, info: Status) -> Outcome<T, (Status, String), ()> {
        match self.err {
            true => Outcome::Failure((info, self.emsg.unwrap())),
            false => Outcome::Success(self.data.unwrap()),
        }
    }
}

impl<T: Serialize> Into<Result<T, String>> for RResult<T> {
    fn into(self) -> Result<T, String> {
        match self.err {
            true => Err(self.emsg.unwrap()),
            false => Ok(self.data.unwrap()),
        }
    }
}
impl<T: Serialize> Into<Option<T>> for RResult<T> {
    fn into(self) -> Option<T> {
        match self.err {
            true =>None,
            false => Some(self.data.unwrap()),
        }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for RResult<T> {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let t = serde_json::to_vec(&self).or_else(|_e| Err(Status::InternalServerError))?;
        let status = if self.err {
            Status::NotAcceptable
        } else {
            Status::Ok
        };
        Response::build()
            .header(ContentType::JSON)
            .sized_body(t.len(), Cursor::new(t))
            .status(status)
            .ok()
    }
}
#[macro_export]
macro_rules! to_rresult {
    (op, $x:expr, $s:expr) => {
        match $x {
            Some(d) => d,
            None => return crate::forms::RResult::err($s),
        }
    };
    (op_rev, $x:expr, $s:expr) => {
        match $x {
            Some(_) => return crate::forms::RResult::err($s),
            None => (),
        }
    };
    (rs, $x:expr) => {
        match $x {
            Ok(d) => d,
            Err(err) => return crate::forms::RResult::err(err),
        }
    };

    (rs, $x:expr, $info:expr) => {
        match $x {
            Ok(d) => d,
            Err(err) => return crate::forms::RResult::err(format!("{} {}", $info, err)),
        }
    };
}
