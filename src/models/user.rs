#[derive(
    serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Identifiable, Default,
)]
pub struct User {
    pub id: u32,
    pub name: String,

    pub email: String,
    pub password: String,
}

use crate::forms::auth::{ChangePaswd, UserAuth};

use super::schema::users;

#[derive(serde::Deserialize, diesel::Insertable)]
#[table_name = "users"]
pub struct NewUser<'s> {
    pub name: &'s str,

    pub email: &'s str,
    pub password: String,
}

impl<'a> NewUser<'a> {
    pub fn from_au_pc(auth: &'a UserAuth, pwdch: &ChangePaswd) -> Self {
        Self {
            name: "",
            email: &auth.email,
            password: pwdch.new.to_string(),
        }
    }
}
