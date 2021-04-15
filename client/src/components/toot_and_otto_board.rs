use crate::{
	toot_and_otto::{
		cpu_toot,
		piece_letter::{PieceLetter, PieceLetter::*},
		player::Player::*,
		toot_and_otto::{TootAndOtto, NUM_COLS, NUM_ROWS},
	},
	types::opponent::Opponent,
};
use serde_json::json;
use strum::IntoEnumIterator;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct TootAndOttoBoard {
	link: ComponentLink<Self>,
	board: TootAndOtto,
	vs: Opponent,
	fetch_task: Option<FetchTask>,
}

pub enum Msg {
	DropPiece(PieceLetter, usize),
	Reset,
	ChangeOpponent(Opponent),
	ReceiveResponse(Result<String, anyhow::Error>),
}

impl TootAndOttoBoard {
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
		let body = &json!({"username": &username, "game": 1, "win": win});
		let request = Request::post("http://localhost:8000/update_score")
			.header("Content-Type", "application/json")
			.body(Json(body))
			.expect("Could not build that request.");
		let callback =
			self.link
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

impl Component for TootAndOttoBoard {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			board: TootAndOtto::new(),
			vs: Opponent::Human,
			fetch_task: None,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::DropPiece(letter, col) => {
				if self.board.is_terminal == true {
					log::info!("Game over");
					return false;
				} else if self.board.drop(letter, col) == false {
					log::info!("Failed to drop");
					return false;
				}

				log::info!("Is game over {}", self.board.is_terminal);
				if self.board.is_terminal {
					match self.board.winner {
						None => {
							// TODO: Insert a tie into the db
							self.update_score(2);
						}
						Some(winner) => match winner {
							OTTO => self.update_score(0),
							TOOT => self.update_score(1),
						},
					}
					return true;
				}

				let cpu_depth = match self.vs {
					Opponent::Human => return true,
					Opponent::EasyCPU => 1,
					Opponent::MediumCPU => 2,
					Opponent::HardCPU => 3,
				};
				let (best_col, best_letter) = cpu_toot::make_move(self.board, cpu_depth);
				self.board.drop(best_letter, best_col);
				if self.board.is_terminal {
					match self.board.winner {
						None => {
							// TODO: Insert a tie into the db
							self.update_score(2);
						}
						Some(winner) => match winner {
							OTTO => self.update_score(0),
							TOOT => self.update_score(1),
						},
					}
					return true;
				}
			}
			Msg::ChangeOpponent(opponent) => {
				if self.board.moves_played == 0 {
					self.vs = opponent;
				}
			}
			Msg::Reset => {
				self.board = TootAndOtto::new();
			}
			Msg::ReceiveResponse(response) => match response.unwrap().as_str() {
				"Update success" => {
					log::info!("Update success");
				}
				_ => {
					log::info!("Update fail");
				}
			},
		}

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		let check_for_piece = move |row: usize, col: usize| -> Html {
			let mut classes = String::from("piece");

			match self.board.check_for_win(TOOT) {
				None => {}
				Some(coordinates) => {
					if coordinates.contains(&[row, col]) {
						classes.push_str(" piece--winner");
					}
				}
			}

			match self.board.check_for_win(OTTO) {
				None => {}
				Some(coordinates) => {
					if coordinates.contains(&[row, col]) {
						classes.push_str(" piece--winner");
					}
				}
			}

			classes.push_str(match self.board.board[row][col] {
				None => " piece--empty",
				Some(letter) => match letter {
					T => " piece--toot-n-otto",
					O => " piece--toot-n-otto",
				},
			});

			let letter = match self.board.board[row][col] {
				None => String::from(""),
				Some(l) => format!("{}", l),
			};

			html! {<div class=classes>{letter}</div>}
		};

		let game_status = move || -> Html {
			let mut arrow_color_class = match self.board.active_player {
				TOOT => "turn__arrow--left",
				OTTO => "turn__arrow--right",
			};

			if self.board.is_terminal {
				arrow_color_class = "turn__arrow--game-over"
			}

			let arrow_text = match self.board.is_terminal {
				false => "",
				true => match self.board.winner {
					Some(winner) => match winner {
						TOOT => "YOU WIN!!!",
						OTTO => "CPU WINS :(",
					},
					None => "TIE GAME!",
				},
			};

			html! {
				<div class="turn">
					<div class="turn__piece turn__player--toot">{"TOOT"}</div>
					<div class=format!("turn__arrow {}", arrow_color_class)>{arrow_text}</div>
					<div class="turn__piece turn__player--otto">{"OTTO"}</div>
				</div>
			}
		};

		let opponent_buttons = move || -> Html {
			html! {
				Opponent::iter().map(|opponent| {
					html! {
						<button
							class=format!("opponent__button {}", if self.vs == opponent {"opponent__button--selected"} else {""})
							onclick=self.link.callback(move |_| Msg::ChangeOpponent(opponent))
						>
							{opponent}
						</button>
					}
				}).collect::<Html>()
			}
		};

		let floating_pieces = move |col: usize| -> Html {
			let show_piece = move |letter| -> &str {
				let player_index = match self.board.active_player {
					TOOT => 0,
					OTTO => 1,
				};

				let letter_index = match letter {
					T => 0,
					O => 1,
				};

				if self.board.piece_counts[player_index][letter_index] == 0 {
					return "piece--floating--hidden";
				} else {
					return "";
				}
			};

			html! {
				PieceLetter::iter().map(|letter| html! {
					<div class="cell cell--floating">
							<div
								class=format!("piece piece--floating {}", show_piece(letter))
								onclick=self.link.callback(move |_| Msg::DropPiece(letter, col))
							>
								{letter}
							</div>
						</div>
				}).collect::<Html>()
			}
		};

		html! {
			<div class="container">
				<div class="board">
					{
						(0..NUM_COLS).into_iter().map(|col| {
							return html! {
								<div class="column">
									{ floating_pieces(col) }
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
				<div class="piece-counts__container">
					<div class="piece-counts__player">
						<p class="piece-counts__player__piece">{format!("TOOT's T's: {}", self.board.piece_counts[0][0])}</p>
						<p class="piece-counts__player__piece">{format!("TOOT's O's: {}", self.board.piece_counts[0][1])}</p>
					</div>
					<div class="piece-counts__player">
						<p class="piece-counts__player__piece">{format!("OTTO's T's: {}", self.board.piece_counts[1][0])}</p>
						<p class="piece-counts__player__piece">{format!("OTTO's O's: {}", self.board.piece_counts[1][1])}</p>
					</div>
				</div>
				{game_status()}
				<div class="dashboard">
					<button class="dashboard__reset" onclick=self.link.callback(move |_| Msg::Reset)>{"RESET"}</button>
					<div class=format!("opponent {}", if self.board.moves_played > 0 { "opponent--disabled" } else { "" })>
						{opponent_buttons()}
					</div>
				</div>
			</div>
		}
	}
}
