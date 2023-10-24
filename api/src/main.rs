#![feature(proc_macro_hygiene, decl_macro)] // Needed for Rocket

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde; // For serializing/deserializing JSON

mod routes; // imports the mod.rs inside the routes directory

use routes::{encryption, decryption};

fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/encrypt", routes![encryption::encrypt])
        .mount("/decrypt", routes![decryption::decrypt])
}


#[rocket::main]
async fn main() {
    let _ = rocket().launch().await;
}

