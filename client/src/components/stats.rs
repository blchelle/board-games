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
}

pub enum Msg {
  GetStats,
  ReceiveResponse(Result<GameInfo, anyhow::Error>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInfo {
  pub username: String,
  pub xo_wins: i32,
  pub xo_total: i32,
  pub to_wins: i32,
  pub to_total: i32,
}

impl Stats {
  fn get_stats(&mut self) {
    // let body = &json!({"username": &self.username});
    let request = Request::get(format!("http://localhost:8000/scores/{}", &self.username))
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
    }
  }

  fn rendered(&mut self, first_render: bool) {
    if first_render && self.username != "" {
      self.get_stats();
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    if self.init {
      log::info!("gettingstats");
      self.get_stats();
      self.init = false;
    }
    match msg {
      Msg::GetStats => {
        self.get_stats();
      }
      Msg::ReceiveResponse(response) => {
        log::info!("{:#?}", &response);
        match response {
          Ok(res) => self.game_info = Some(res),
          Err(_e) => self.game_info = None,
        }
      }
    }
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    if self.game_info.is_none() {
      return html! {<div class="stats_header">{"Please log in to see stats"}</div>};
    }
    let gi = self.game_info.as_ref().unwrap();
    if gi.username == "" {
      return html! {
        <div>{"Please log in to see stats"}</div>
      };
    }
    html! {
      <div class="stats_page">
        <h1 class="stats_header">{format!("Game stats for {}", gi.username)}</h1>
        <h2 class="stats_entry">{format!("Connect 4 wins {}", gi.xo_wins)}</h2>
        <h2 class="stats_entry">{format!("Connect 4 losses {}", gi.xo_total - gi.xo_wins)}</h2>
        <h2 class="stats_entry">{format!("Toot and Otto wins {}", gi.to_wins)}</h2>
        <h2 class="stats_entry">{format!("Toot and Otto losses {}", gi.to_total - gi.to_wins)}</h2>
      </div>
    }
  }
}
