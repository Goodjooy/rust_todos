use crate::forms::{self, auth::UserAuth, user_detail::JUserDetail};

use super::schema::uesr_details;

#[derive(Queryable, Identifiable)]
#[table_name = "uesr_details"]
pub struct UserDetail {
    pub id: u32,
    pub uid: u32,

    pub signature: String,
}

impl Default for UserDetail {
    fn default() -> Self {
        Self {
            id: Default::default(),
            uid: Default::default(),
            signature: "什么都没有留下~~".into(),
        }
    }
}

impl UserDetail {
    pub fn into_jdetail(self) -> JUserDetail {
        JUserDetail {
            signature: self.signature,
        }
    }
}

#[derive(serde::Deserialize, Insertable)]
#[table_name = "uesr_details"]
pub struct SetDetail<'s> {
    pub uid: u32,
    pub signature: &'s str,
}

impl Default for SetDetail<'_> {
    fn default() -> Self {
        Self {
            uid: Default::default(),
            signature: "什么都没有留下~~".into(),
        }
    }
}

impl<'s> SetDetail<'s> {
    pub fn new_def(uid: u32) -> Self {
        Self {
            uid,
            ..Default::default()
        }
    }
    pub fn into_jdetail(self)->JUserDetail{
        JUserDetail { signature: self.signature.into() }
    }

    pub fn from_uath(auser: &UserAuth, data: &'s forms::user_detail::JUserDetail) -> Option<Self> {
        Some(Self {
            uid: auser.get_id()?,
            signature: &data.signature,
        })
    }
}

impl Into<JUserDetail> for UserDetail {
    fn into(self) -> JUserDetail {
        self.into_jdetail()
    }
}

impl <'s> Into<JUserDetail> for SetDetail<'s>  {
    fn into(self) -> JUserDetail {
        self.into_jdetail()
    }
}