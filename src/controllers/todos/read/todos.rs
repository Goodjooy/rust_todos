use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::TextExpressionMethods;
use rocket::State;
use rocket::{post, Data, UriDisplayPath};

use crate::forms::todo;
use crate::load;
use crate::{
    database::DatabaseConnection,
    forms::{
        auth::UserAuth,
        todo::{
            fromforms::{PageRange, TagFilter, TodoFilter},
            JTodo,
        },
        RResult,
    },
    to_rresult,
};

//TODO 信息读取

//#[post("todo/?<page>&<tag_filter>&<filter>")]
pub fn get_todos(
    user: UserAuth,
    page: Option<PageRange>,
    tag_filter: Option<TagFilter>,
    filter: Option<TodoFilter>,
    db: &State<DatabaseConnection>,
) -> RResult<Vec<JTodo>> {
    use crate::models::schema::todo_infos::dsl::*;

    todo!()
}

// tag id and tag name
fn tag_bound(
    user: &UserAuth,
    tag_filter: Option<TagFilter>,
    db: &State<DatabaseConnection>,
) -> RResult<Vec<(u32, String)>> {
    use crate::models::schema::todo_tags::dsl::*;
    let db = to_rresult!(rs, db.get());
    let tag_names = to_rresult!(op, tag_filter, "Empty data");
    let tag_names=tag_names.tgkw.iter();

    let res = tag_names
        .map(|n| {
            todo_tags
                .select((id, name))
                .filter(name.eq(n))
                .filter(uid.eq(user.get_id().unwrap()))
                .first::<(u32, String)>(&*db)
        })
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect();

    RResult::ok(res)
}
// todo id and todo title
fn bound(
    user: &UserAuth,
    filter: Option<TodoFilter>,
    db: &State<DatabaseConnection>,
) -> RResult<Vec<(u32, String)>> {
    use crate::models::schema::todo_infos::dsl::*;
    let db = to_rresult!(rs, db.get());
    let key_word = to_rresult!(op, filter, "Empty data").tkw;

    let res = todo_infos
        .select((id, title))
        .filter(uid.eq(user.get_id().unwrap()))
        .filter(title.like(format!("%{}%", key_word)))
        .load::<(u32, String)>(&*db);

    let res = to_rresult!(rs, res);
    RResult::ok(res)
}

fn both_match_bound(
    tag_names: &Vec<(u32, String)>,
    todo_names: &Vec<(u32, String)>,
    db: &State<DatabaseConnection>,
) ->RResult<Vec<u32>>{
    use crate::models::schema::todo_tag_links::dsl::*;
    let db = to_rresult!(rs, db.get());

    todo!()

}
