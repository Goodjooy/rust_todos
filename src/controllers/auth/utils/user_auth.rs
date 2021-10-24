use std::sync::Mutex;

use diesel::{mysql, QueryDsl, RunQueryDsl};

use crate::{database::DatabaseConnection, forms::auth::UserAuth, models::user::User};

use super::AuthError;

impl UserAuth {
    pub fn get_id(&self) -> Option<u32> {
        let res = &self.id?;
        Some(*res)
    }

    pub fn into_full_user<'a>(
        &self,
        db: &'a DatabaseConnection,
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
