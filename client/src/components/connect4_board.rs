use crate::{
	connect4::{
		connect4::{Connect4, NUM_COLS, NUM_ROWS},
		cpu,
		piece_color::{PieceColor, PieceColor::*},
	},
	types::opponent::Opponent,
};
use serde_json::json;
use strum::IntoEnumIterator;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Connect4Board {
	board: Connect4,
	vs: Opponent,
	link: ComponentLink<Self>,
	fetch_task: Option<FetchTask>,
}

pub enum Msg {
	DropPiece(usize),
	Reset,
	ChangeOpponent(Opponent),
	ReceiveResponse(Result<String, anyhow::Error>),
}

impl Connect4Board {
	fn update_score(&mut self, win: u8) {
		let ls = web_sys::window().unwrap().local_storage().unwrap().unwrap();
		let username = match ls.get_item("user_logged_in") {
			Ok(a) => match a {
				Some(b) => b,
				None => "".to_string(),
			},
			Err(_) => "".to_string(),
		};
		if username == "" {
			return;
		}
		let body = &json!({"username": &username, "game": 0, "win": win});
		let request = Request::post("http://localhost:8000/update_score")
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

impl Component for Connect4Board {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			board: Connect4::new(),
			vs: Opponent::HardCPU,
			fetch_task: None,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::DropPiece(col) => {
				if let Some(_) = self.board.winner {
					return false;
				} else if self.board.moves_played == 42 {
					return false;
				}

				if self.board.drop(col) == false {
					return false;
				}
				if let Some(winner) = self.board.winner {
					log::info!("{}", winner);
					// player is red
					// update game score
					match winner {
						PieceColor::RED => self.update_score(1),
						PieceColor::YELLOW => self.update_score(0),
					}
					return true;
				} else if self.board.winner.is_none() && self.board.is_terminal {
					self.update_score(2);
				}

				let depth = match self.vs {
					Opponent::Human => return true,
					Opponent::EasyCPU => 1,
					Opponent::MediumCPU => 2,
					Opponent::HardCPU => 3,
				};

				self.board.drop(cpu::make_move(self.board, depth));

				if let Some(winner) = self.board.winner {
					log::info!("{}", winner);
					// player is red
					// update game score
					match winner {
						PieceColor::RED => self.update_score(1),
						PieceColor::YELLOW => self.update_score(0),
					}
					return true;
				} else if self.board.winner.is_none() && self.board.is_terminal {
					self.update_score(2);
				}
			}
			Msg::Reset => {
				self.board = Connect4::new();
			}
			Msg::ChangeOpponent(opponent) => {
				if self.board.moves_played == 0 {
					self.vs = opponent;
				}
			}
			Msg::ReceiveResponse(response) => match response.unwrap().as_str() {
				"Update success" => {
					log::info!("Update success");
				}
				_ => {
					log::info!("Update fail");
				}
			},
		};

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		true
	}

	fn view(&self) -> Html {
		let floating_piece_letter = move || -> &str {
			match self.board.active_player {
				RED => "R",
				YELLOW => "Y",
			}
		};

		let check_for_piece = move |row: usize, col: usize| -> Html {
			let mut classes = String::from("piece");

			if let Some(_) = self.board.winner {
				if self
					.board
					.check_for_win(self.board.active_player.switch())
					.unwrap()
					.contains(&[row, col])
				{
					classes.push_str(" piece--winner");
				}
			}

			classes.push_str(match self.board.board[row][col] {
				None => " piece--empty",
				Some(color) => match color {
					RED => " piece--red",
					YELLOW => " piece--yellow",
				},
			});

			let letter = match self.board.board[row][col] {
				None => "",
				Some(color) => match color {
					RED => "R",
					YELLOW => "Y",
				},
			};

			html! {<div class=classes>{letter}</div>}
		};

		let game_status = move || -> Html {
			let mut arrow_color_class = match self.board.active_player {
				RED => "turn__arrow--red",
				YELLOW => "turn__arrow--yellow",
			};

			if self.board.is_terminal {
				arrow_color_class = "turn__arrow--game-over"
			}

			let arrow_text = match self.board.is_terminal {
				false => "",
				true => match self.board.winner {
					Some(winner) => match winner {
						RED => "YOU WIN!!!",
						YELLOW => "CPU WINS :(",
					},
					None => "TIE GAME!",
				},
			};

			html! {
				<div class="turn">
					<div class="turn__piece turn__piece--red">{"R"}</div>
					<div class=format!("turn__arrow {}", arrow_color_class)>{arrow_text}</div>
					<div class="turn__piece turn__piece--yellow">{"Y"}</div>
				</div>
			}
		};

		let opponent_buttons = move || -> Html {
			html! {
				<div class=format!("opponent {}", if self.board.moves_played > 0 { "opponent--disabled" } else { "" }) >
					{
						Opponent::iter().map(|opponent| {
							html! {
								<button
									class=format!("opponent__button {}", if self.vs == opponent {"opponent__button--selected"} else {""})
									onclick=self.link.callback(move |_| Msg::ChangeOpponent(opponent))
								>
									{opponent}
								</button>
						}}).collect::<Html>()
					}
				</div>
			}
		};

		let floating_piece_class = move || -> &str {
			match self.board.active_player {
				RED => "piece--red",
				YELLOW => "piece--yellow",
			}
		};

		html! {
			<div class="container">
				<div class="board">
				{
					(0..NUM_COLS).into_iter().map(|col| {
						return html! {
							<div class="column" onclick=self.link.callback(move |_| Msg::DropPiece(col))>
								<div class="cell cell--floating">
									<div class={format!("piece piece--hidden {}", floating_piece_class())}>{floating_piece_letter()}</div>
								</div>
								{
									(0..NUM_ROWS).into_iter().map(|row| {
										return html! {
											<div class="cell">
												{check_for_piece(row, col)}
											</div>
										}
									}).collect::<Html>()
								}
							</div>
						}
					}).collect::<Html>()
				}
				</div>
				{game_status()}
				<div class="dashboard">
					<button onclick=self.link.callback(move |_| Msg::Reset)>{"RESET"}</button>
					{opponent_buttons()}
				</div>
			</div>
		}
	}
}
