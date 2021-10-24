use crate::DatabaseConnection;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, http::Status, post, response::content, serde::json::Json, State};

use crate::{
    forms::{auth::UserAuth, user_detail::JUserDetail},
    generate_controller,
    models::user_detail::{SetDetail, UserDetail},
};

generate_controller!(UserDetailCtrl, "/user", set_detail, load_detail);

#[post("/detail", data = "<input>")]
fn set_detail(
    auth: UserAuth,
    input: Json<JUserDetail>,
    db: &State<DatabaseConnection>,
) -> Option<(Status, String)> {
    let i_detail = SetDetail::from_uath(&auth, &input);

    let res = {
        use crate::models::schema::user_details::dsl::*;
        let db = db.lock().ok()?;

        if let Ok(ud) = user_details
            .filter(uid.eq(auth.get_id()?))
            .first::<UserDetail>(&*db)
        {
            diesel::update(&ud)
                .set(signature.eq(&input.signature))
                .execute(&*db)
        } else {
            diesel::insert_into(user_details)
                .values(&i_detail)
                .execute(&*db)
        }
    }
    .ok()?;
    Some((
        Status::Ok,
        format!("updata info success({}) [ {} ]", res, &input.signature),
    ))
}
#[get("/detail")]
fn load_detail(
    auser: UserAuth,
    db: &State<DatabaseConnection>,
) -> Option<rocket::response::content::Json<JUserDetail>> {
    use crate::models::schema::user_details::dsl::*;

    let user_id = auser.get_id()?;
    let db = db.get().ok()?;

    let det = user_details
        .filter(uid.eq(user_id))
        .first::<UserDetail>(&db)
        .and_then(|f| Ok(f.into_jdetail()))
        .unwrap_or_else(|_| {
            let seter = SetDetail::new_def(user_id);
            diesel::insert_into(user_details)
                .values(&seter)
                .execute(&db)
                .expect("Failure Insert data");
            seter.into_jdetail()
        });

    Some(content::Json(det))
}
