use crate::new_data_struct;

use super::super::schema::todo_tags;

use super::super::schema::todo_tag_links;

new_data_struct!(
    TodoTag,
    NewTag,
    's,
    "todo_tags",
    u32,
    items=>[
        uid:u32|u32,
        name: String |&'s str
    ]
);

new_data_struct!(TodoTagLink,
    NewLink,
    "todo_tag_links",
    u32,
    items=>[
    tid:u32 | u32,
    gid:u32 | u32
]);
