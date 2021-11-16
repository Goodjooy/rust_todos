use crate::utils::LenLimitedString;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct UserAuth {
    pub id :Option<u32>,
    pub email: LenLimitedString<128>,
    pub paswd: LenLimitedString<64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ChangePaswd {
    pub old:LenLimitedString<64>,
    pub new:LenLimitedString<64>,
    pub new_conf:LenLimitedString<64>
}
