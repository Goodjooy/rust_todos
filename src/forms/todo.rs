use chrono::NaiveDate;
use rocket::{
    data::{FromData, ToByteUnit},
    http::Status,
    outcome::Outcome,
};

pub mod fromforms;
#[derive(serde::Deserialize,serde::Serialize)]
pub struct JTodo {
    pub title: String,
    pub descript: String,
    pub tags: Vec<String>,
    pub ddl: NaiveDate,
}
#[derive(serde::Deserialize)]
struct JTodoMid {
    pub title: String,
    pub descript: String,
    pub tags: Vec<String>,
    pub ddl: String,
}

impl JTodoMid {
    fn into(self) -> Option<JTodo> {
        Some(JTodo {
            title: self.title.clone(),
            descript: self.descript.clone(),
            tags: self.tags,
            ddl: NaiveDate::parse_from_str(&self.ddl, "%Y-%m-%d").ok()?,
        })
    }
}

impl JTodo {
    fn into(self) -> JTodoMid {
        JTodoMid {
            title: self.title,
            descript: self.descript,
            tags: self.tags,
            ddl: self.ddl.format("%Y-%d-%m").to_string(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for JTodo {
    type Error = String;

    async fn from_data(
        _req: &'r rocket::Request<'_>,
        data: rocket::Data<'r>,
    ) -> Outcome<Self, (rocket::http::Status, Self::Error), rocket::Data<'r>> {
        let reader = data.open(512.kibibytes());
        let slent = reader.into_bytes().await;
        let slent = match slent {
            Ok(s) => s,
            Err(_) => {
                return Outcome::Failure((Status::NotAcceptable, "Failure Load Data".to_string()))
            }
        };

        let res = serde_json::from_slice::<JTodoMid>(&slent);

        let res = match res {
            Ok(res) => res,
            Err(_) => return Outcome::Failure((Status::NotAcceptable, "Not a Json".to_string())),
        };

        match res.into() {
            Some(r) => Outcome::Success(r),
            None => Outcome::Failure((
                Status::NotAcceptable,
                "Wroing Time Pattern : %Y-%m-%d".to_string(),
            )),
        }
    }
}
