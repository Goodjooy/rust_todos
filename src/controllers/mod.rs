use std::vec;

use diesel::sql_types;

pub mod auth;
pub mod todos;
pub mod user_detail;

no_arg_sql_function!(last_insert_id, sql_types::Bigint, "LAST_INSERT_ID()");
pub trait Controller {
    fn routes() -> Vec<rocket::Route> {
        vec![]
    }
    fn base<'s>() -> &'s str {
        "/"
    }
}
#[macro_export]
macro_rules! generate_controller {
    ($name:ident,$base:literal,$($routes:ident),*) => {
        pub struct $name;
        impl crate::controllers::Controller for $name  {
            fn routes()->Vec<rocket::Route>{
                rocket::routes![
                    $(
                        $routes
                    ),*
                ]
            }

            fn base<'s>()->&'s str{
                $base
            }
        }
    };
}
