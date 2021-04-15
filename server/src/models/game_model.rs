/*
Database calls to update scores and get scores
*/
use crate::MyMongo;
use bson::doc;

// Game struct
#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct Game {
	pub username: String,
	pub xo_wins: i32,
	pub xo_loss: i32,
	pub xo_ties: i32,
	pub to_wins: i32,
	pub to_loss: i32,
	pub to_ties: i32,
}

// Update score struct
#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct ScoreUpdate {
	pub username: String,
	pub game: u8, // 0 connect 4, 1 toot
	pub win: u8,  // 0 loss, 1 win, 2 tie
}

impl MyMongo {
	// DB function to update the score
	pub fn update_score(
		&mut self,
		username: &String,
		game: u8,
		win: u8,
	) -> Result<bool, mongodb::error::Error> {
		let score_db = self.db.collection("scores");
		let score = match game {
			0 => {
				if win == 1 {
					doc! {
						"xo_wins": 1,
					}
				} else if win == 0 {
					doc! {
						"xo_loss": 1
					}
				} else {
					doc! {
						"xo_ties": 1,
					}
				}
			}
			1 => {
				if win == 1 {
					doc! {
						"to_wins": 1,
					}
				} else if win == 0 {
					doc! {
						"to_loss": 1
					}
				} else {
					doc! {
						"to_ties": 1,
					}
				}
			}
			_ => return Ok(false),
		};
		score_db.update_one(
			doc! {
				"username": username
			},
			doc! {"$inc": score},
			None,
		)?;
		Ok(true)
	}

	// DB function to get the game scores
	pub fn get_game_score(
		&mut self,
		username: String,
	) -> Result<Option<Game>, mongodb::error::Error> {
		let score_db = self.db.collection("scores");
		let res = score_db.find_one(doc! {"username": &username}, None)?;
		match res {
			Some(r) => {
				let gi = Game {
					username: username,
					xo_wins: r.get("xo_wins").unwrap().as_i32().unwrap_or(-1),
					xo_loss: r.get("xo_loss").unwrap().as_i32().unwrap_or(-1),
					xo_ties: r.get("xo_ties").unwrap().as_i32().unwrap_or(-1),
					to_wins: r.get("to_wins").unwrap().as_i32().unwrap_or(-1),
					to_loss: r.get("to_loss").unwrap().as_i32().unwrap_or(-1),
					to_ties: r.get("to_ties").unwrap().as_i32().unwrap_or(-1),
				};
				return Ok(Some(gi));
			}
			None => return Ok(None),
		}
	}
}
