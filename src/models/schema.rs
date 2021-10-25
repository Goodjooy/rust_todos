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
    uesr_details,
    users,
);
