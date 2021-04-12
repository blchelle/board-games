#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use bson::{bson, doc};
use mongodb::{options::ClientOptions, Client};
use rocket::http::Method::{Get, Post};
use rocket::http::RawStr;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};
mod database;
#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}
#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct GameInfo {
    pub username: String,
    pub xo_wins: i32,
    pub xo_total: i32,
    pub to_wins: i32,
    pub to_total: i32,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/new_user", format = "application/json", data = "<user>")]
fn new_user(user: Json<UserInfo>) -> Json<String> {
    match database::MyMongo::new() {
        Ok(mut db) => {
            match db.add_user(&user.username, &user.password) {
                Ok(()) => {}
                Err(e) => return Json(String::from("Username taken")),
            };
        }
        Err(e) => return Json(String::from("Error connecting to database")),
    }
    Json(String::from(format!("Created user: {}", user.username)))
}

#[post("/login", format = "application/json", data = "<user>")]
fn login(user: Json<UserInfo>) -> Json<String> {
    match database::MyMongo::new() {
        Ok(mut db) => match db.login(&user.username, &user.password) {
            Ok(res) => {
                if res == false {
                    return Json(String::from("Login failed"));
                }
            }
            Err(e) => return Json(String::from("Login failed")),
        },
        Err(e) => return Json(String::from("Error connecting to database")),
    }
    Json(String::from(format!("Login success")))
}

#[get("/scores/<username>")]
fn get_scores(username: &RawStr) -> Json<String> {
    match database::MyMongo::new() {
        Ok(mut db) => {
            match db.get_game_score(username.to_string()) {
                Ok(r) => return Json(serde_json::to_string(&r).unwrap()),
                Err(e) => return Json(String::from("User not found")),
            };
        }
        Err(e) => return Json(String::from("Error connecting to database")),
    }
    Json(String::from(format!("Unexpected error")))
}

fn main() {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(vec![Get, Post].into_iter().map(From::from).collect())
        .allow_credentials(true);
    rocket::ignite()
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![index, new_user, login, get_scores])
        .launch();
}
