#[derive(serde::Deserialize,serde::Serialize,rocket::response::Responder)]
pub struct JUserDetail {
    pub signature:String,
}