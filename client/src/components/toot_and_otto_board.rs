use crate::{
	toot_and_otto::{
		piece_letter::{PieceLetter, PieceLetter::*},
		player::{Player, Player::*},
		toot_and_otto::{TootAndOtto, NUM_COLS, NUM_ROWS},
	},
	types::opponent::Opponent,
};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use yew::prelude::*;

pub struct TootAndOttoBoard {
	link: ComponentLink<Self>,
	active_player: Player,
	board: TootAndOtto,
	winner: Option<Player>,
	tie: bool,
	turn_number: usize,
	piece_counts: HashMap<Player, HashMap<PieceLetter, usize>>,
	vs: Opponent,
}

pub enum Msg {
	DropPiece(PieceLetter, usize),
	Reset,
	ChangeOpponent(Opponent),
}

impl Component for TootAndOttoBoard {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		let mut letter_counts = HashMap::new();
		letter_counts.insert(O, 6);
		letter_counts.insert(T, 6);

		let mut player_piece_counts = HashMap::new();
		player_piece_counts.insert(TOOT, letter_counts.clone());
		player_piece_counts.insert(OTTO, letter_counts.clone());

		Self {
			link,
			active_player: TOOT,
			board: TootAndOtto::new(),
			winner: None,
			tie: false,
			turn_number: 1,
			piece_counts: player_piece_counts,
			vs: Opponent::Human,
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
				} else if self.piece_counts[&self.active_player][&letter] == 0 {
					return false;
				}

				// Updates the piece count for the player
				*self
					.piece_counts
					.get_mut(&self.active_player)
					.unwrap()
					.get_mut(&letter)
					.unwrap() -= 1;

				let toot_won = self.board.check_for_win(TOOT);
				let otto_won = self.board.check_for_win(OTTO);

				self.winner = match (toot_won, otto_won) {
					(true, true) => {
						self.tie = true;
						None
					}
					(false, true) => Some(OTTO),
					(true, false) => Some(TOOT),
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
			Msg::ChangeOpponent(opponent) => {
				self.vs = opponent;
			}
			Msg::Reset => {
				let mut letter_counts = HashMap::new();
				letter_counts.insert(O, 6);
				letter_counts.insert(T, 6);

				let mut player_piece_counts = HashMap::new();
				player_piece_counts.insert(TOOT, letter_counts.clone());
				player_piece_counts.insert(OTTO, letter_counts.clone());
				self.piece_counts = player_piece_counts;

				self.active_player = TOOT;
				self.board = TootAndOtto::new();
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
					T => html! {<div class="piece piece--toot-n-otto">{"T"}</div>},
					O => html! {<div class="piece piece--toot-n-otto">{"O"}</div>},
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
					TOOT => html! {<p>{"TOOT (Player 1) Wins!"}</p>},
					OTTO => html! {<p>{"OTTO (Player 2) Wins!"}</p>},
				},
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
				if self.piece_counts[&self.active_player][&letter] == 0 {
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
						<p class="piece-counts__player__piece">{format!("TOOT's T's: {}", self.piece_counts[&TOOT][&T])}</p>
						<p class="piece-counts__player__piece">{format!("TOOT's O's: {}", self.piece_counts[&TOOT][&O])}</p>
					</div>
					<div class="piece-counts__player">
						<p class="piece-counts__player__piece">{format!("OTTO's T's: {}", self.piece_counts[&OTTO][&T])}</p>
						<p class="piece-counts__player__piece">{format!("OTTO's O's: {}", self.piece_counts[&OTTO][&O])}</p>
					</div>
				</div>
				<div class="opponent">
					{opponent_buttons()}
				</div>
				<div class="dashboard">
					<button onclick=self.link.callback(move |_| Msg::Reset)>{"Reset Game"}</button>
					{game_status()}
				</div>
			</div>
		}
	}
}
