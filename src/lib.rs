#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;
extern crate rocket_cors;
extern crate dotenv;
extern crate mongodb;
extern crate r2d2;
extern crate r2d2_mongodb;

extern crate serde;

#[macro_use] extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins,  // 2.
    Cors, CorsOptions // 3.
};

use rocket::http::Method; // 1.
use dotenv::dotenv;
use rocket::{Request, Rocket};
pub mod cats;
// mod mongo_connection;
mod static_files;
pub mod spotify;



fn make_cors() -> Cors {
    // let allowed_origins = AllowedOrigins::some_exact(&[ // 4.
    //     "http://localhost:8080",
    //     "http://localhost:8001",
    //     "http://127.0.0.1:8080",
    //     "http://localhost:8000",
    //     "http://0.0.0.0:8000",
    //     "https://accounts.spotify.com",
    // ]);
    CorsOptions { // 5.
        allowed_origins: AllowedOrigins::some_exact(&[ // 4.
            "http://localhost:8080",
            "http://localhost:8001",
            "https://accounts.spotify.com",
        ]),
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
            "Origin",
            "Content-type" // 6.
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}


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
                static_files::callback,
                static_files::github
            ],
        )
        .attach(make_cors())
}
