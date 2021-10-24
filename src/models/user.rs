#[derive(serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Identifiable)]
pub struct User {
    pub id: u32,
    pub name: String,

    pub email: String,
    pub password: String,
}

use super::schema::users;

#[derive(serde::Deserialize, diesel::Insertable)]
#[table_name = "users"]
pub struct NewUser<'s> {
    pub name: &'s str,

    pub email: &'s str,
    pub password: String,
}

