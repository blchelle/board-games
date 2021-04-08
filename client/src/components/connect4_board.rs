use crate::connect4::connect4::{Connect4, PieceColor, NUM_COLS, NUM_ROWS};

use yew::prelude::*;

pub struct Connect4Board {
	board: Connect4,
	active_player: PieceColor,
	winner: Option<PieceColor>,
	link: ComponentLink<Self>,
}

pub enum Msg {
	DropPiece(usize),
}

impl Component for Connect4Board {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			active_player: PieceColor::YELLOW,
			board: Connect4::new(),
			winner: None,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::DropPiece(col) => {
				if let Some(_) = self.winner {
					return false;
				}

				self.board.drop(self.active_player, col);

				match self.board.check_for_win(self.active_player) {
					false => self.winner = None,
					true => self.winner = Some(self.active_player),
				}

				self.active_player = self.active_player.switch();
			}
		};

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		let check_for_piece = move |row: usize, col: usize| -> Html {
			match self.board.board[row][col] {
				None => html! {<div class="piece piece--empty"></div>},
				Some(color) => match color {
					PieceColor::RED => html! {<div class="piece piece--red">{"R"}</div>},
					PieceColor::YELLOW => html! {<div class="piece piece--yellow">{"Y"}</div>},
				},
			}
		};

		html! {
			<div class="connect4-board">
				{
					(0..NUM_COLS).into_iter().map(|col| {
						return html! {
							<div class="connect4-column" onclick=self.link.callback(move |_| Msg::DropPiece(col))>
								{
									(0..NUM_ROWS).into_iter().map(|row| {
										return html! {
											<div class="connect4-cell">{check_for_piece(row, col)}</div>
										}
									}).collect::<Html>()
								}
							</div>
						}
					}).collect::<Html>()
				}
			</div>
		}
	}
}
