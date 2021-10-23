#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct UserAuth {
    pub id :Option<u32>,
    pub email: String,
    pub paswd: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ChangePaswd {
    pub old:String,
    pub new:String,
    pub new_conf:String
}