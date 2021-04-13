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

pub struct LoginPage {
  link: ComponentLink<Self>,
  username: String,
  password: String,
  fetch_task: Option<FetchTask>,
}

pub enum Msg {
  Login(bool),
  Logout,
  UpdateUsername(String),
  UpdatePassword(String),
  ReceiveResponse(Result<String, anyhow::Error>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInfo {
  pub username: String,
  pub xo_wins: i32,
  pub xo_total: i32,
  pub to_wins: i32,
  pub to_total: i32,
}

impl Component for LoginPage {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      link: link,
      username: "".to_string(),
      password: "".to_string(),
      fetch_task: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Login(login) => {
        log::info!("{:#?}", self.username);
        log::info!("{:#?}", self.password);
        let body = &json!({"username": &self.username, "password": &self.password});
        let request = Request::post(format!(
          "http://localhost:8000/{}",
          if login { "login" } else { "new_user" }
        ))
        .header("Content-Type", "application/json")
        .body(Json(body))
        .expect("Could not build that request.");
        let callback =
          self
            .link
            .callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
              let Json(data) = response.into_body();
              Msg::ReceiveResponse(data)
            });
        // 3. pass the request and callback to the fetch service
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        // 4. store the task so it isn't canceled immediately
        self.fetch_task = Some(task);
      }
      Msg::UpdateUsername(username) => self.username = username,
      Msg::UpdatePassword(password) => self.password = password,
      Msg::ReceiveResponse(response) => {
        let window = web_sys::window().unwrap();
        let ls = window.local_storage().unwrap().unwrap();
        match response.unwrap().as_str() {
          "Login success" => {
            ls.set_item("user_logged_in", &self.username)
              .expect("Error setting user login");
            log::info!("logged in as {:#?}", &self.username);
            let document = window.document().unwrap();
            let location = document.location().unwrap();
            let url = format!(
              "{}//{}/{}",
              location.protocol().expect("error"),
              location.host().expect("error"),
              "connect-4/"
            );
            location.set_href(&url).expect("failed");
          }
          _ => {
            ls.set_item("user_logged_in", &"")
              .expect("Error clearing user login");
            log::info!("login fail");
          }
        }
      }
      Msg::Logout => {
        let window = web_sys::window().unwrap();
        let ls = window.local_storage().unwrap().unwrap();
        ls.set_item("user_logged_in", &"")
          .expect("Error clearing user login");
      }
    }
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let window = web_sys::window().unwrap();
    let ls = window.local_storage().unwrap().unwrap();
    let username = ls.get_item("user_logged_in").unwrap().unwrap();
    let login_status = move || -> Html {
      match username.as_str() {
        "" => html! {
          <>
            <input type="text" name="username" id="login-username" placeholder="Username" class="login-form-field" value={&self.username}
              oninput=self.link.callback(|e: InputData| Msg::UpdateUsername(e.value))/>
            <input type="password" name="password" id="login-password" placeholder="Password" class="login-form-field"
              oninput=self.link.callback(|e: InputData| Msg::UpdatePassword(e.value))/>
            <button id="login-submit" onclick=self.link.callback(move |_| Msg::Login(true))>{"Login"}</button>
            <button id="login-submit" onclick=self.link.callback(move |_| Msg::Login(false))>{"Sign up"}</button>
          </>
        },
        _ => html! {
          <>
            <div>{format!("Logged in as {}", username)}</div>
            <button id="login-submit" onclick=self.link.callback(move |_| Msg::Logout)>{"Logout"}</button>
          </>
        },
      }
    };

    html! {
      <div class="login-page">
          {login_status()}
      </div>
    }
  }
}
