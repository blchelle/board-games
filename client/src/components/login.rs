/*
Login component for client
*/
use serde_json::json;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

// Login page struct
pub struct LoginPage {
  link: ComponentLink<Self>,
  username: String,
  error: Option<String>,
  password: String,
  fetch_task: Option<FetchTask>,
}

// Message passing
pub enum Msg {
  Login(bool),
  Logout,
  UpdateUsername(String),
  UpdatePassword(String),
  ReceiveResponse(Result<String, anyhow::Error>),
}

impl LoginPage {
  // Send login request to the server and establish a call back
  pub fn login(&mut self, login: bool) {
    if self.username == "" || self.password == "" {
      return;
    }
    let body = &json!({"username": &self.username, "password": &self.password});
    let request = Request::post(format!(
      "http://localhost:8000/{}",
      if login { "login" } else { "new_user" }
    ))
    .header("Content-Type", "application/json")
    .body(Json(body))
    .expect("Could not build that request.");
    let callback = self
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
}

impl Component for LoginPage {
  type Message = Msg;
  type Properties = ();
  // Create component
  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      link: link,
      error: None,
      username: "".to_string(),
      password: "".to_string(),
      fetch_task: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Login(login) => {
        if self.username.len() == 0 || self.password.len() == 0 {
          return false;
        }

        if self.username.contains(" ") {
          self.error = Some(String::from("Invalid username"));
          return true;
        }

        // Send request to server
        self.login(login);
      }
      Msg::UpdateUsername(username) => {
        self.username = username;
        self.error = None;
      }
      Msg::UpdatePassword(password) => {
        self.password = password;
        self.error = None;
      }
      Msg::ReceiveResponse(response) => {
        // Parse response from server
        let window = web_sys::window().unwrap();
        let ls = window.local_storage().unwrap().unwrap();
        match response.unwrap().as_str() {
          "Login success" => {
            // Add logged in user to local storage
            ls.set_item("user_logged_in", &self.username)
              .expect("Error setting user login");

            // Navigate to connect 4 page
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
          "Created user" => {
            // Add logged in user to local storage
            ls.set_item("user_logged_in", &self.username)
              .expect("Error setting user login");

            // Navigate to connect 4 page
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
          "Login failed" => self.error = Some(String::from("Failed to login")),
          "Username taken" => self.error = Some(String::from("This username is already taken")),
          _ => {
            // Clear user login
            ls.set_item("user_logged_in", &"")
              .expect("Error clearing user login");
          }
        }
      }
      Msg::Logout => {
        // Logout and clear local storage
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
    let username = match ls.get_item("user_logged_in") {
      Ok(a) => match a {
        Some(b) => b,
        None => "".to_string(),
      },
      Err(_) => "".to_string(),
    };

    let submit_button_class = if self.username.len() > 0 && self.password.len() > 0 {
      ""
    } else {
      "button--disabled"
    };

    let login_status = move || -> Html {
      match username.as_str() {
        "" => html! {
          <>
            <h4>
              {"Enter your authentication credentials below and login or create an account with them"}
            </h4>
            <div class="auth__container auth__container--username">
            <label class={if self.username.len() > 0 {"raised"} else {""}}>
              {"Username"}
            </label>
              <input
                type="text"
                name="username"
                id="login-username"
                value={&self.username}
                oninput=self.link.callback(|e: InputData| Msg::UpdateUsername(e.value))/>
            </div>
            <div class="auth__container auth__container--password">
              <label class={if self.password.len() > 0 {"raised"} else {""}}>
                {"Password"}
              </label>
              <input
                type="password"
                name="password"
                id="login-password"
                oninput=self.link.callback(|e: InputData| Msg::UpdatePassword(e.value))/>
            </div>
            {
              html! {
                if let Some(message) = &self.error {
                  html! {<p class="auth__error">{message}</p>}
                } else {
                  html! {}
                }
              }
            }
            <button
                class=submit_button_class
                id="button--login"
                onclick=self.link.callback(move |_| Msg::Login(true))
            >
                {"Login"}
            </button>
            <div class="separator__container">
              <div class="separator__line"></div>
              <p class="separator__label">{"OR"}</p>
              <div class="separator__line"></div>
            </div>
            <button
                class=submit_button_class
                id="button--signup"
                onclick=self.link.callback(move |_| Msg::Login(false))
            >
                {"Sign up"}
            </button>
          </>
        },
        _ => html! {
          <>
            <div class="login-header">{format!("Logged in as {}", username)}</div>
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
