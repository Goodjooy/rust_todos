table! {
    users (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        signature -> Nullable<Text>,
        email -> Varchar,
        password -> Varchar,
    }
}
