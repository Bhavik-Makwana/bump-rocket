#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate mongodb;
extern crate r2d2;
extern crate r2d2_mongodb;
#[macro_use] extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;
use rocket::{Request, Rocket};
pub mod cats;
// mod mongo_connection;
use rocket_contrib::databases;
mod static_files;
#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}
#[database("mongodb_logs")]
struct LogsDbConn(mongodb::db::Database);

#[catch(400)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

pub fn rocket() -> Rocket {
    dotenv().ok();
    rocket::ignite()
        .register(catchers![internal_error, not_found])
        .attach(LogsDbConn::fairing())
        .mount(
            "/api/v1",
            routes![
                // cats::handler::all,
                // cats::handler::get,
                // cats::handler::post,
                // cats::handler::put,
                // cats::handler::delete,
                // cats::handler::delete_all,
                static_files::all,
                static_files::index,
                static_files::login,
                static_files::callback
            ],
        )
}
