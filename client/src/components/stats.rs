use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{
  format::{Json, Nothing},
  prelude::*,
};
use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

pub struct Stats {
  link: ComponentLink<Self>,
  username: String,
  fetch_task: Option<FetchTask>,
  game_info: Option<GameInfo>,
  init: bool,
  search: String,
}

pub enum Msg {
  GetStats,
  ReceiveResponse(Result<GameInfo, anyhow::Error>),
  UpdateSearch(String),
  Search,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInfo {
  pub username: String,
  pub xo_wins: i32,
  pub xo_loss: i32,
  pub xo_ties: i32,
  pub to_wins: i32,
  pub to_loss: i32,
  pub to_ties: i32,
}

impl Stats {
  fn get_stats(&mut self, user: String) {
    // let body = &json!({"username": &self.username});
    if user == "" {
      return;
    }
    let request = Request::get(format!("http://localhost:8000/scores/{}", &user))
      .header("Content-Type", "application/json")
      .body(Nothing)
      .expect("Could not build that request.");
    let callback = self.link.callback(
      |response: Response<Json<Result<GameInfo, anyhow::Error>>>| {
        let Json(data) = response.into_body();
        Msg::ReceiveResponse(data)
      },
    );
    // 3. pass the request and callback to the fetch service
    let task = FetchService::fetch(request, callback).expect("failed to start request");
    // 4. store the task so it isn't canceled immediately
    self.fetch_task = Some(task);
  }
}

impl Component for Stats {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let ls = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let username = match ls.get_item("user_logged_in") {
      Ok(a) => match a {
        Some(b) => b,
        None => "".to_string(),
      },
      Err(_) => "".to_string(),
    };
    Self {
      link: link,
      username: username,
      fetch_task: None,
      game_info: None,
      init: true,
      search: "".to_string(),
    }
  }

  fn rendered(&mut self, first_render: bool) {
    if first_render && self.username != "" {
      let user = self.username.to_string();
      self.get_stats(user);
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    if self.init {
      log::info!("gettingstats");
      let user = self.username.to_string();
      self.get_stats(user);
      self.init = false;
    }
    match msg {
      Msg::GetStats => {
        let user = self.username.to_string();
        self.get_stats(user);
      }
      Msg::ReceiveResponse(response) => {
        log::info!("{:#?}", &response);
        match response {
          Ok(res) => self.game_info = Some(res),
          Err(_e) => self.game_info = None,
        }
      }
      Msg::UpdateSearch(search) => self.search = search,
      Msg::Search => {
        let user = self.search.to_string();
        self.get_stats(user);
      }
    }
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    let mut stat_results = html! {};

    let ls = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let username = match ls.get_item("user_logged_in") {
      Ok(a) => match a {
        Some(b) => b,
        None => "".to_string(),
      },
      Err(_) => "".to_string(),
    };
    if !self.game_info.as_ref().is_none() {
      let gi = self.game_info.as_ref().unwrap();

      if gi.username == "" {
        stat_results = html! {
          <h1 class="stats_header">{"No stats found for search result"}</h1>
        };
      } else {
        stat_results = html! {
          <div class="stats">
            <h1 class="stats_header">{format!("Game stats for {}", gi.username)}</h1>
            <h2 class="stats_entry">{format!("Connect 4 wins {}", gi.xo_wins)}</h2>
            <h2 class="stats_entry">{format!("Connect 4 losses {}", gi.xo_loss)}</h2>
            <h2 class="stats_entry">{format!("Connect 4 ties {}", gi.xo_ties)}</h2>
            <h2 class="stats_entry">{format!("Toot and Otto wins {}", gi.to_wins)}</h2>
            <h2 class="stats_entry">{format!("Toot and Otto losses {}", gi.to_loss)}</h2>
            <h2 class="stats_entry">{format!("Toot and Otto ties {}", gi.to_ties)}</h2>
          </div>
        }
      }
    } else if username == "" {
      stat_results = html! {
        <h1 class="stats_header">{"Please log in to see stats or search"}</h1>
      };
    }
    html! {
      <div class="stats_page">
        <div class="search">
          <input type="text" name="Search" id="search" placeholder="username" value={&self.search}
            oninput=self.link.callback(|e: InputData| Msg::UpdateSearch(e.value))/>
          <button onclick=self.link.callback(move |_| Msg::Search)>{"Search"}</button>
        </div>
        {stat_results}
      </div>
    }
  }
}
