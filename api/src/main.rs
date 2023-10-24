#![feature(proc_macro_hygiene, decl_macro)] // Needed for Rocket

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde; // For serializing/deserializing JSON

mod routes; // imports the mod.rs inside the routes directory

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::http::Method;
use routes::{encryption, decryption};

// Your CORS Fairing
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.adjoin_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.adjoin_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.adjoin_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.adjoin_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// Catch-all OPTIONS route
#[options("/<_path..>")]
pub fn handle_options<'r>(_path: std::path::PathBuf) -> Status {
    Status::NoContent
}

// Rocket builder function
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/encrypt", routes![encryption::encrypt])
        .mount("/decrypt", routes![decryption::decrypt])
        .mount("/", routes![handle_options])
        .attach(CORS)
}

// Main function
#[rocket::main]
async fn main() {
    let _ = rocket().launch().await;
}
