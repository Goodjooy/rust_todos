#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct UserAuth {
    pub id :Option<u32>,
    pub email: String,
    pub paswd: String,
}
