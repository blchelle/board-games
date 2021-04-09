use crate::components::{connect4_board::Connect4Board, navbar::NavBar};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Switch, Clone)]
pub enum Routes {
	#[to = "/signup"]
	SignUp,
	#[to = "/login"]
	Login,
	#[to = "/"]
	Connect4,
}

pub struct App {}

pub enum Msg {}

impl Component for App {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		Self {}
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender {
		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<div class="app">
				<NavBar />
				<Router<Routes, ()>
					render = Router::render(|switch: Routes| {
						match switch {
							Routes::Connect4 => html! {<Connect4Board />},
							Routes::Login => html! {<Connect4Board />},
							Routes::SignUp => html! {<Connect4Board />},
						}
					})
				/>
			</div>
		}
	}
}
