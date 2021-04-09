#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::http::RawStr;
use rocket_contrib::json::Json;
use mongodb::{Client, options::ClientOptions};
use bson::{doc, bson};
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
    pub to_total: i32
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/new_user", format="json", data="<user>")]
fn new_user(user: Json<UserInfo>) -> String {

    match database::MyMongo::new() {
        Ok(mut db) => {
            match db.add_user(&user.username, &user.password) {
                Ok(()) => {},
                Err(e) => {return String::from("Username taken")} 
            };

        },
        Err(e) => {return String::from("Error connecting to database")}
    }
    String::from(format!("Created user: {}", user.username))
} 

#[post("/login", format="json", data="<user>")]
fn login(user: Json<UserInfo>) -> String {
    match database::MyMongo::new() {
        Ok(mut db) => {
            match db.login(&user.username, &user.password) {
                Ok(res) => {
                    if res == false {
                        return String::from("Login failed");
                    }
                },
                Err(e) => {return String::from("Login failed")} 
            }
        },
        Err(e) => {return String::from("Error connecting to database")}
    }
    String::from(format!("Login success"))
} 

#[get("/scores/<username>")]
fn get_scores(username: &RawStr) -> String {
    match database::MyMongo::new() {
        Ok(mut db) => {
            match db.get_game_score(username.to_string()) {
                Ok(r) => {return serde_json::to_string(&r).unwrap()},
                Err(e) => {return String::from("User not found")} 
            };

        },
        Err(e) => {return String::from("Error connecting to database")}
    }
    String::from(format!("Unexpected error"))
}

fn main() {
    
    // init_mongo();
    rocket::ignite().mount("/", routes![index, new_user, login, get_scores]).launch();
}
