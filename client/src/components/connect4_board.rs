use crate::{
	connect4::{
		connect4::{Connect4, NUM_COLS, NUM_ROWS},
		cpu,
		piece_color::{PieceColor, PieceColor::*},
	},
	types::opponent::Opponent,
};

use strum::IntoEnumIterator;
use yew::prelude::*;

pub struct Connect4Board {
	board: Connect4,
	active_player: PieceColor,
	vs: Opponent,
	link: ComponentLink<Self>,
}

pub enum Msg {
	DropPiece(usize),
	Reset,
	ChangeOpponent(Opponent),
}

impl Component for Connect4Board {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			active_player: RED,
			board: Connect4::new(),
			vs: Opponent::HardCPU,
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

				if self.board.drop(self.active_player, col) == false {
					return false;
				}

				if let Some(_) = self.board.winner {
					return true;
				}

				self.active_player = self.active_player.switch();

				match self.vs {
					Opponent::Human => return true,
					Opponent::EasyCPU => {
						self.board
							.drop(self.active_player, cpu::make_move(self.board, 1));
					}
					Opponent::MediumCPU => {
						self.board
							.drop(self.active_player, cpu::make_move(self.board, 5));
					}
					Opponent::HardCPU => {
						self.board
							.drop(self.active_player, cpu::make_move(self.board, 15));
					}
				};

				self.active_player = self.active_player.switch();
			}
			Msg::Reset => {
				self.active_player = RED;
				self.board = Connect4::new();
			}
			Msg::ChangeOpponent(opponent) => {
				if self.board.moves_played == 0 {
					self.vs = opponent;
				}
			}
		};

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
					RED => html! {<div class="piece piece--red">{"R"}</div>},
					YELLOW => html! {<div class="piece piece--yellow">{"Y"}</div>},
				},
			}
		};

		let game_status = move || -> Html {
			if self.board.moves_played >= 42 {
				return html! {<p>{"It's a draw!"}</p>};
			}

			match self.board.winner {
				None => {
					html! {<p>{format!("Turn {}, {}'s Move", self.board.moves_played, self.active_player)}</p>}
				}
				Some(winner) => match winner {
					RED => html! {<p>{"Player 1 Wins!"}</p>},
					YELLOW => html! {<p>{"Player 2 Wins!"}</p>},
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

		html! {
			<div class="container">
				<div class="board">
				{
					(0..NUM_COLS).into_iter().map(|col| {
						return html! {
							<div class="column" onclick=self.link.callback(move |_| Msg::DropPiece(col))>
							{
								(0..NUM_ROWS).into_iter().map(|row| {
									return html! {
										<div class="cell">{check_for_piece(row, col)}</div>
									}
								}).collect::<Html>()
							}
							</div>
						}
					}).collect::<Html>()
				}
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
