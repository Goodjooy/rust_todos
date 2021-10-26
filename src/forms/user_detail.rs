use crate::models::user::User;

#[derive(serde::Deserialize, serde::Serialize, rocket::response::Responder)]
pub struct JUserDetail {
    pub signature: String,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct JUserInfo {
    pub name: String,
    pub email: String,
    pub signature: String,
}

impl JUserInfo {
    pub fn from_u_ud(user: &User, detail: &JUserDetail) -> Self {
        Self {
            name: user.name.to_string(),
            email: user.email.to_string(),
            signature: detail.signature.to_string(),
        }
    }
}
