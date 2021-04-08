use crate::connect4::connect4::{Connect4, PieceColor, NUM_COLS, NUM_ROWS};

use yew::prelude::*;

pub struct NavBar {
	link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for NavBar {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { link }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<div class="navbar">
				<h4>{"Board Games"}</h4>
				<div class="navbar__links">
					<a class="navbar__link" href="#">{"TOOT AND OTTO"}</a>
					<a class="navbar__link" href="#">{"LOG IN"}</a>
				</div>
			</div>
		}
	}
}
