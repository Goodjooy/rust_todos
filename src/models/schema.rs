table! {
    todo_infos (id) {
        id -> Unsigned<Integer>,
        uid -> Unsigned<Integer>,
        title -> Varchar,
        descript -> Text,
        ddl -> Datetime,
    }
}

table! {
    todo_tag_links (id) {
        id -> Unsigned<Integer>,
        tid -> Unsigned<Integer>,
        gid -> Unsigned<Integer>,
    }
}

table! {
    todo_tags (id) {
        id -> Unsigned<Integer>,
        uid -> Unsigned<Integer>,
        name -> Varchar,
    }
}

table! {
    uesr_details (id) {
        id -> Unsigned<Integer>,
        uid -> Unsigned<Integer>,
        signature -> Text,
    }
}

table! {
    users (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    todo_infos,
    todo_tag_links,
    todo_tags,
    uesr_details,
    users,
);
