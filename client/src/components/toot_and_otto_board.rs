use crate::toot_and_otto::toot_and_otto::{PieceLetter, Player, TootAndOtto, NUM_COLS, NUM_ROWS};
use yew::prelude::*;

pub struct TootAndOttoBoard {
	link: ComponentLink<Self>,
	active_player: Player,
	board: TootAndOtto,
	winner: Option<Player>,
	tie: bool,
	turn_number: usize,
}

pub enum Msg {
	DropPiece(PieceLetter, usize),
	Reset,
}

impl Component for TootAndOttoBoard {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			active_player: Player::TOOT,
			board: TootAndOtto::new(),
			winner: None,
			tie: false,
			turn_number: 1,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::DropPiece(letter, col) => {
				if let Some(_) = self.winner {
					return false;
				} else if self.board.drop(letter, col) == false {
					return false;
				} else if self.tie == true {
					return false;
				}

				let toot_won = self.board.check_for_win(Player::TOOT);
				let otto_won = self.board.check_for_win(Player::OTTO);

				self.winner = match (toot_won, otto_won) {
					(true, true) => {
						self.tie = true;
						None
					}
					(false, true) => Some(Player::OTTO),
					(true, false) => Some(Player::TOOT),
					(false, false) => None,
				};

				if let Some(_) = self.winner {
					return true;
				} else if self.tie == true {
					return true;
				}

				self.turn_number += 1;
				if self.turn_number > NUM_COLS * NUM_ROWS {
					self.tie = true
				}

				self.active_player = self.active_player.switch();
			}
			Msg::Reset => {
				self.board = TootAndOtto::new();
				self.active_player = Player::TOOT;
				self.winner = None;
				self.tie = false;
				self.turn_number = 1;
			}
		}

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		true
	}

	fn view(&self) -> Html {
		let check_for_piece = move |row: usize, col: usize| -> Html {
			match self.board.board[row][col] {
				None => html! {<div class="piece piece--empty"></div>},
				Some(color) => match color {
					PieceLetter::T => html! {<div class="piece piece--toot-n-otto">{"T"}</div>},
					PieceLetter::O => html! {<div class="piece piece--toot-n-otto">{"O"}</div>},
				},
			}
		};

		let game_status = move || -> Html {
			if self.tie {
				return html! {<p>{"It's a draw!"}</p>};
			}

			match self.winner {
				None => {
					html! {<p>{format!("Turn {}, {}'s Move", self.turn_number, self.active_player)}</p>}
				}
				Some(winner) => match winner {
					Player::TOOT => html! {<p>{"TOOT (Player 1) Wins!"}</p>},
					Player::OTTO => html! {<p>{"OTTO (Player 2) Wins!"}</p>},
				},
			}
		};

		html! {
			<div class="container">
				<div class="board">
					{
						(0..NUM_COLS).into_iter().map(|col| {
							return html! {
								<div class="column">
									<div class="cell cell--floating">
										<div
											class="piece piece--floating"
											onclick=self.link.callback(move |_| Msg::DropPiece(PieceLetter::T, col))
										>
										{"T"}
										</div>
									</div>
									<div class="cell cell--floating">
										<div
											class="piece piece--floating"
											onclick=self.link.callback(move |_| Msg::DropPiece(PieceLetter::O, col))
										>
											{"O"}
										</div>
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
				<div class="dashboard">
					<button onclick=self.link.callback(move |_| Msg::Reset)>{"Reset Game"}</button>
					{game_status()}
				</div>
			</div>
		}
	}
}
