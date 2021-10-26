use database::DatabaseConnection;

use controllers::{auth::UserAuthCtrl, user_detail::UserDetailCtrl, Controller};

#[macro_use]
extern crate diesel;
extern crate rocket;
extern crate serde;

mod controllers;
mod database;
mod forms;
mod models;

#[rocket::launch]
fn launch() -> _ {
    rocket::build()
        //.manage(connect_pool())
        .manage(DatabaseConnection::new().expect("DB Connect can not establish"))
        .mount(UserAuthCtrl::base(), UserAuthCtrl::routes())
        .mount(UserDetailCtrl::base(), UserDetailCtrl::routes())
}
