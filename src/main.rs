use std::sync::Mutex;
use std::env;

use diesel::{mysql, Connection, MysqlConnection};


use controllers::{Controller, auth::UserAuthCtrl};

#[macro_use]
extern crate diesel;
extern crate rocket;

mod models;
mod controllers;
mod forms;

fn connect_database() -> mysql::MysqlConnection {
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL Not Found");

    MysqlConnection::establish(&db_url)
    .expect(&format!("Failure To Connect To DB {}", db_url))
}

#[rocket::launch]
fn launch()->_{
    rocket::build()
    .manage(Mutex::new(connect_database()))
    .mount(UserAuthCtrl::base(),UserAuthCtrl::routes())

}