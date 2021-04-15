/*
Routes to update and get game scores
*/
use crate::{
	models::game_model::{Game, ScoreUpdate},
	MyMongo,
};
use rocket::http::RawStr;
use rocket_contrib::json::Json;

// Post request to update game scores
#[post("/update_score", format = "application/json", data = "<score>")]
pub fn update_score(score: Json<ScoreUpdate>) -> Json<String> {
	match MyMongo::new() {
		// Establish connection
		Ok(mut db) => match db.update_score(&score.username, score.game, score.win) {
			// Update score
			Ok(res) => {
				if res {
					return Json(String::from("Update success"));
				} else {
					return Json(String::from("Update failed"));
				}
			}
			Err(_) => return Json(String::from("Update failed")),
		},
		Err(_) => return Json(String::from("Update failed")),
	}
}

// Get request to obtain user stats
#[get("/scores/<username>")]
pub fn get_scores(username: &RawStr) -> Json<Game> {
	let err = Game {
		// error struct. Sends this if no user found
		username: "".to_string(),
		xo_wins: -1,
		xo_loss: -1,
		xo_ties: -1,
		to_wins: -1,
		to_loss: -1,
		to_ties: -1,
	};
	match MyMongo::new() {
		// Establish connection
		Ok(mut db) => {
			match db.get_game_score(username.to_string()) {
				// Gets the game score
				Ok(r) => {
					if r.is_none() {
						return Json(err);
					} else {
						return Json(r.unwrap());
					}
				}
				Err(_) => return Json(err),
			};
		}
		Err(_) => return Json(err),
	}
}
