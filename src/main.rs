#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod models;
pub mod routes;
pub mod schema;

#[database("owo_faces")]
pub struct DbConn(rocket_contrib::databases::diesel::MysqlConnection);

fn main() {
    rocket::ignite()
        .register(catchers![
            routes::not_found,
            routes::internal_server
        ])
        .mount("/", routes![
            routes::faces,
            routes::search_face,
            routes::new,
            routes::account
        ])
        .attach(DbConn::fairing())
        .launch();
}
