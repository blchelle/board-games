use super::*;
use bson::{bson, doc};
use mongodb::{options::ClientOptions, sync::Client, sync::Database};

pub struct MyMongo {
  conn: Client,
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
    Ok(MyMongo { conn: conn, db: db })
  }

  pub fn add_user(
    &mut self,
    username: &String,
    password: &String,
  ) -> Result<(), mongodb::error::Error> {
    let userdb = self.db.collection("users");
    let user = doc! {"username" : username, "password": password};
    userdb.insert_one(user, None)?;

    let scoredb = self.db.collection("scores");
    let score = doc! {
      "username": username,
      "xo_wins": 0,
      "xo_ties": 0,
      "xo_loss": 0,
      "to_wins": 0,
      "to_ties": 0,
      "to_loss": 0
    };
    scoredb.insert_one(score, None)?;
    Ok(())
  }

  pub fn update_score(
    &mut self,
    username: &String,
    game: u8,
    win: u8,
  ) -> Result<bool, mongodb::error::Error> {
    let scoredb = self.db.collection("scores");
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

    scoredb.update_one(
      doc! {
        "username": username
      },
      doc! {"$inc": score},
      None,
    )?;
    Ok(true)
  }

  pub fn login(
    &mut self,
    username: &String,
    password: &String,
  ) -> Result<bool, mongodb::error::Error> {
    let userdb = self.db.collection("users");
    let user = doc! {"username" : username, "password": password};
    let res = userdb.find_one(user, None)?;
    match res {
      Some(r) => return Ok(true),
      None => return Ok(false),
    }
  }

  pub fn get_game_score(
    &mut self,
    username: String,
  ) -> Result<Option<GameInfo>, mongodb::error::Error> {
    let scoredb = self.db.collection("scores");
    let res = scoredb.find_one(doc! {"username": &username}, None)?;

    match res {
      Some(r) => {
        let gi = GameInfo {
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
