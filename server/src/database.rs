use mongodb::{Client, options::ClientOptions, Database};
use bson::{doc, bson};
use super::*;
pub struct MyMongo {
  conn: Client,
  db: Database
}

impl MyMongo {
  pub fn new() -> Result<MyMongo, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;
    // Manually set an option.
    client_options.app_name = Some("Server".to_string());
    // Get a handle to the deployment.
    let conn = Client::with_options(client_options)?;
    let db = conn.database("421ServerDB");
    Ok(MyMongo {
        conn: conn,
        db: db
    })
  }

  pub fn add_user(&mut self, username: &String, password: &String) -> Result<(), mongodb::error::Error> {
    let userdb = self.db.collection("users");
    let user = doc! {"username" : username, "password": password};
    userdb.insert_one(user, None)?;

    let scoredb = self.db.collection("scores");
    let score = doc! {
      "username": username,
      "xo_wins": 0,
      "xo_total": 0,
      "to_wins": 0,
      "to_total": 0
    };
    scoredb.insert_one(score, None)?;
    Ok(())
  }

  pub fn login(&mut self, username: &String, password: &String) -> Result<bool, mongodb::error::Error> {
    let userdb = self.db.collection("users");
    let user = doc! {"username" : username, "password": password};
    let res = userdb.find_one(user, None)?;
    match res {
      Some(r) => {
        return Ok(true)
      },
      None => return Ok(false)
    }
  }

  pub fn update_game_score(&mut self, gameInfo: GameInfo) -> Result<(), mongodb::error::Error> {
    let scoredb = self.db.collection("scores");
    let score = doc! {
      "xo_wins": gameInfo.xo_wins,
      "xo_total": gameInfo.xo_total,
      "to_wins": gameInfo.to_wins,
      "to_total": gameInfo.to_total
    };
    scoredb.update_one(doc! {
      "username": gameInfo.username
    }, doc! {"$set": score}, None)?;
    Ok(())

  }

  pub fn get_game_score(&mut self, username: String) -> Result<Option<GameInfo>, mongodb::error::Error> {
    let scoredb = self.db.collection("scores");
    let res = scoredb.find_one(doc! {"username": &username}, None)?;

    match res {
      Some(r) => {
        let gi = GameInfo {
          username: username,
          xo_wins: r.get("xo_wins").unwrap().as_i32().unwrap_or(-1),
          xo_total: r.get("xo_total").unwrap().as_i32().unwrap_or(-1),
          to_wins: r.get("to_wins").unwrap().as_i32().unwrap_or(-1),
          to_total: r.get("to_total").unwrap().as_i32().unwrap_or(-1),
        };
        return Ok(Some(gi));
      },
      None => return Ok(None)
    }
  }
}
