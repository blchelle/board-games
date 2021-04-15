#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod models {
    pub mod game_model;
    pub mod user_model;
}

mod controllers {
    pub mod game_controller;
    pub mod user_controller;
}

use controllers::{game_controller, user_controller};
use mongodb::{options::ClientOptions, sync::Client, sync::Database};
use rocket::http::Method::{Get, Post};
use rocket_cors::{AllowedOrigins, CorsOptions};

pub struct MyMongo {
    db: Database,
}

impl MyMongo {
    pub fn new() -> Result<MyMongo, mongodb::error::Error> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;

        // Manually set an option.
        client_options.app_name = Some("Server".to_string());

        // Get a handle to the deployment.
        let conn = Client::with_options(client_options)?;
        let db = conn.database("421ServerDB");

        Ok(MyMongo { db: db })
    }
}

fn main() {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(vec![Get, Post].into_iter().map(From::from).collect())
        .allow_credentials(true);

    rocket::ignite()
        .attach(cors.to_cors().unwrap())
        .mount(
            "/",
            routes![
                user_controller::new_user,
                user_controller::login,
                game_controller::get_scores,
                game_controller::update_score
            ],
        )
        .launch();
}
