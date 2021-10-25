use crate::{first_or_create, forms::RResult, to_rresult, DatabaseConnection};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, post, serde::json::Json, State};

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
) -> RResult<String> {
    let i_detail = SetDetail::from_uath(&auth, &input);

    let res = to_rresult!(rs, {
        use crate::models::schema::uesr_details::dsl::*;
        let db = to_rresult!(rs, db.get());

        crate::update_first_or_create!(
            &db,
            UserDetail,
            uesr_details,
            i_detail,
            filter => [
                uid.eq(to_rresult!(op, auth.get_id(), "AUTH ID Not exist"))
                ],
            set => [
                signature.eq(&input.signature)
                ]
        )
    });
    RResult::ok(format!(
        "updata info success({}) [ {} ]",
        res, &input.signature
    ))
}
#[get("/detail")]
fn load_detail(auser: UserAuth, db: &State<DatabaseConnection>) -> RResult<JUserDetail> {
    use crate::models::schema::uesr_details::dsl::*;

    let user_id = to_rresult!(op, auser.get_id(), "unKnow Auth ID");
    let db = to_rresult!(rs, db.get());

    let det: JUserDetail = first_or_create!(
        &db,
        UserDetail,
        uesr_details,
        SetDetail::new_def(user_id),
        uid.eq(user_id)
    );
    RResult::ok(det)
}

