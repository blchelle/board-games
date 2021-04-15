use crate::{models::user_model::User, MyMongo};

use rocket_contrib::json::Json;

#[post("/new_user", format = "application/json", data = "<user>")]
pub fn new_user(user: Json<User>) -> Json<String> {
	match MyMongo::new() {
		Ok(mut db) => {
			match db.add_user(&user.username, &user.password) {
				Ok(()) => {}
				Err(_) => return Json(String::from("Username taken")),
			};
		}
		Err(_) => return Json(String::from("Error connecting to database")),
	}
	Json(String::from(format!("Created user")))
}

#[post("/login", format = "application/json", data = "<user>")]
pub fn login(user: Json<User>) -> Json<String> {
	match MyMongo::new() {
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
