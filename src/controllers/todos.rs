use crate::{
    database::DatabaseConnection,
    forms::{auth::UserAuth, todo::JTodo},
    models::todo::{NewLink, NewTag, NewTodo, TodoTag},
    to_rresult,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{post, State};

use crate::{forms::RResult, generate_controller};

generate_controller!(TodoCtrl, "/todos", new_todo);

#[post("/todo", data = "<data>")]
fn new_todo(data: JTodo, user: UserAuth, db: &State<DatabaseConnection>) -> RResult<String> {
    // add new tags
    let all_tags = {
        let db = to_rresult!(rs, db.get());
        use crate::models::schema::todo_tags::dsl::*;
        let res = data
            .tags
            .iter()
            .map(|t| {
                let in_data = NewTag::from((&user, t.as_str()));
                // check target tag is exist ornot
                match todo_tags.filter(name.eq(t)).first::<TodoTag>(&*db) {
                    Ok(data) => Some(data),
                    Err(_) => {
                        // not exist ,create this tag
                        diesel::insert_into(todo_tags)
                            .values(&in_data)
                            .execute(&*db)
                            .ok()?;
                        // get the id of newest id
                        let newid: u32 = todo_tags
                            .select(super::last_insert_id)
                            .first::<i64>(&*db)
                            .ok()?
                            .try_into()
                            .ok()?;
                        // trans new tag into todo tag
                        let new: TodoTag = (in_data, newid).into();
                        Some(new)
                    }
                }
            })
            // filter tag create failure
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .collect::<Vec<_>>();
        res
    };
    // add todo
    let new_id = {
        use crate::models::schema::todo_infos::dsl::*;
        let db = to_rresult!(rs, db.get());

        let in_todo = NewTodo::from((&user, &data));
        // insert new todo
        to_rresult!(
            rs,
            diesel::insert_into(todo_infos)
                .values(&in_todo)
                .execute(&*db)
        );
        // get the insert id
        to_rresult!(
            rs,
            todo_infos.select(super::last_insert_id).first::<i64>(&*db)
        )
    };
    let new_id: u32 = to_rresult!(rs, new_id.try_into());

    // link todo with tags
    {
        let db = to_rresult!(rs, db.get());
        use crate::models::schema::todo_tag_links::dsl::*;

        // general todo-tag linkers
        let linkers = all_tags
            .iter()
            .map(|t| &t.id)
            .map(|t| NewLink::from((t, &new_id)))
            .collect::<Vec<_>>();

        to_rresult!(
            rs,
            diesel::insert_into(todo_tag_links)
                .values(linkers)
                .execute(&*db)
        );
    }

    RResult::ok("Save Todo Info Done".into())
}
