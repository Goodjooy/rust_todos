use std::boxed::Box;

use rocket::{http::Status, outcome::Outcome, request::FromRequest, Request};

use crate::forms::{RResult, auth::UserAuth};

use super::{auth_key::AuthKey, COOKIE_NAME};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuth {
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, (Status, Self::Error), ()> {
        let jar = request.cookies();
        if let Some(cookie) = jar.get_private(COOKIE_NAME) {
            let user = AuthKey::<UserAuth>::from_cookie(cookie, COOKIE_NAME, jar);
            let rr_user=RResult::from_option(user, "Auth Token Has Been Unavailable");

            rr_user.into_outcome(Status::Unauthorized)
        } else {
            Outcome::Failure((Status::Unauthorized, "Auth Token Not Exist".into()))
        }
    }

    type Error = String;
}
