/*
Database calls to verify login, and add new users
*/
use crate::MyMongo;
use bson::doc;

#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct User {
	pub username: String,
	pub password: String,
}

impl MyMongo {
	// Adds a user to the database
	pub fn add_user(
		&mut self,
		username: &String,
		password: &String,
	) -> Result<(), mongodb::error::Error> {
		let user_db = self.db.collection("users");
		let user = doc! {"username" : username, "password": password};
		user_db.insert_one(user, None)?;
		let score_db = self.db.collection("scores");
		let score = doc! {
			"username": username,
			"xo_wins": 0,
			"xo_ties": 0,
			"xo_loss": 0,
			"to_wins": 0,
			"to_ties": 0,
			"to_loss": 0
		};
		score_db.insert_one(score, None)?;
		Ok(())
	}

	// Checks if user exists in the database
	pub fn login(
		&mut self,
		username: &String,
		password: &String,
	) -> Result<bool, mongodb::error::Error> {
		let user_db = self.db.collection("users");
		let user = doc! {"username" : username, "password": password};
		let res = user_db.find_one(user, None)?;
		match res {
			Some(_) => return Ok(true),
			None => return Ok(false),
		}
	}
}
