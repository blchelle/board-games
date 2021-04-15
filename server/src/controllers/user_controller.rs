/*
Routes to login and create a new user
*/
use crate::{models::user_model::User, MyMongo};

use rocket_contrib::json::Json;

// Post request to create a new user
#[post("/new_user", format = "application/json", data = "<user>")]
pub fn new_user(user: Json<User>) -> Json<String> {
	match MyMongo::new() {
		// Establish connection
		Ok(mut db) => {
			match db.add_user(&user.username, &user.password) {
				// Add user to db
				Ok(()) => {}
				Err(_) => return Json(String::from("Username taken")),
			};
		}
		Err(_) => return Json(String::from("Error connecting to database")),
	}
	Json(String::from(format!("Created user")))
}

// Post request to verify login request
#[post("/login", format = "application/json", data = "<user>")]
pub fn login(user: Json<User>) -> Json<String> {
	match MyMongo::new() {
		// Establish connection
		Ok(mut db) => match db.login(&user.username, &user.password) {
			Ok(res) => {
				if res == false {
					return Json(String::from("Login failed"));
				}
			}
			Err(_) => return Json(String::from("Login failed")),
		},
		Err(_) => return Json(String::from("Error connecting to database")),
	}
	Json(String::from(format!("Login success")))
}
