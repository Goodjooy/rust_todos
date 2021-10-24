use std::sync::Mutex;

use diesel::{mysql, QueryDsl, RunQueryDsl};

use crate::{database::DatabaseConnection, models::user::NewUser};
const EMAIL_REGEX: &str = r#"^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$"#;
impl NewUser<'_> {
    pub fn encode_password<F>(&self, handle: F) -> Self
    where
        F: Fn(&str) -> String,
    {
        Self {
            name: self.name,
            email: self.email,
            password: handle(&self.password),
        }
    }
    pub fn check_able(&self, db: &DatabaseConnection) -> bool {
        check_email_avaiable(self.email)
            && check_email_used(self.email, db)
            && check_name_size(self.name)
            && check_password_size(&self.password)
    }
}
pub fn check_email_used(mail: &str, db: &DatabaseConnection) -> bool {
    use crate::models::schema::users::dsl::*;

    let db = db.lock().unwrap();
    use diesel::ExpressionMethods;
    let res = users.filter(email.eq(mail)).count().get_result::<i64>(&*db);

    if let Ok(num) = res {
        if num == 0 {
            true
        } else {
            false
        }
    } else {
        true
    }
}
pub fn check_email_avaiable(email: &str) -> bool {
    let reg = regex::Regex::new(EMAIL_REGEX).unwrap();

    reg.is_match(email) && email.len() <= 128
}

pub fn check_name_size(name: &str) -> bool {
    name.len() <= 32
}
pub fn check_password_size(passwd: &str) -> bool {
    match passwd.len() {
        s if (s < 8 || s > 64) => false,
        _ => true,
    }
}
