use super::schema::todo_infos;

mod impls;
mod tags;

pub use impls::*;
pub use tags::*;

crate::new_data_struct!(
    TodoInfo,
    NewTodo,
    's,
    "todo_infos",
    u32,
    items=>[
        uid: u32|u32,
        title: String|& 's str,
        descript: String|& 's str,
        ddl: chrono::NaiveDateTime|chrono::NaiveDateTime
    ]
);
