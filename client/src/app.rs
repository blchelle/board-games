use crate::components::{connect4_board::Connect4Board, navbar::NavBar};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Switch, Clone, PartialEq)]
pub enum AppRoute {
	#[to = "/signup"]
	SignUp,
	#[to = "/login"]
	Login,
	#[to = "/connect-4"]
	Connect4,
	#[to = "/toot-n-otto"]
	TootAndOtto,
	#[to = "/"]
	Home,
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
		true
	}

	fn view(&self) -> Html {
		html! {
			<div class="app">
				<NavBar />
				<Router<AppRoute, ()>
					render = Router::render(|switch: AppRoute| {
						match switch {
							AppRoute::Home => html! {<Connect4Board/>},
							AppRoute::Connect4 => html! {<Connect4Board />},
							AppRoute::Login => html! {<div>{"Login"}</div>},
							AppRoute::SignUp => html! {<Connect4Board />},
							AppRoute::TootAndOtto => html! {<Connect4Board />},
							_ => html! {<Connect4Board />}
						}
					})
				/>
			</div>
		}
	}
}
