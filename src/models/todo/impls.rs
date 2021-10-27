use crate::models::todo::NewLink;
use crate::models::todo::NewTag;
use crate::models::todo::TodoInfo;
use crate::models::todo::TodoTag;
use chrono::NaiveDateTime;
use chrono::NaiveTime;

use crate::{
    forms::{auth::UserAuth, todo::JTodo},
    models::todo::NewTodo,
};

impl<'s> From<(&UserAuth, &'s JTodo)> for NewTodo<'s> {
    fn from(d: (&UserAuth, &'s JTodo)) -> Self {
        Self {
            uid: d.0.get_id().unwrap_or(0),
            title: &d.1.title,
            descript: &d.1.descript,
            ddl: {
                let d = d.1.ddl.clone();
                NaiveDateTime::new(d,NaiveTime::from_hms(0,0,0))
            },
        }
    }
}

impl NewTodo<'_> {
    pub fn into_jtodo(self, tags: &[String]) -> JTodo {
        JTodo {
            title: self.title.to_string(),
            descript: self.descript.to_string(),
            tags: Vec::from_iter(tags.iter().map(|s| s.to_owned())),
            ddl: self.ddl.date(),
        }
    }
}

impl TodoInfo {
    pub fn into_jtodo(self, tags: &[String]) -> JTodo {
        JTodo {
            title: self.title,
            descript: self.descript,
            tags: tags.iter().map(String::to_owned).collect(),
            ddl: self.ddl.date(),
        }
    }
}

impl From<(NewTag<'_>, u32)> for TodoTag {
    fn from((t, i): (NewTag<'_>, u32)) -> Self {
        TodoTag {
            id: i,
            uid: t.uid,
            name: t.name.to_string(),
        }
    }
}

impl<'s> From<(&UserAuth, &'s str)> for NewTag<'s> {
    fn from((u, s): (&UserAuth, &'s str)) -> Self {
        Self {
            uid: u.get_id().unwrap_or(0),
            name: s,
        }
    }
}

impl From<(&u32, &u32)> for NewLink {
    fn from((tid, oid): (&u32, &u32)) -> Self {
        Self {
            tid: *tid,
            gid: *oid,
        }
    }
}
