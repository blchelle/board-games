use crate::{
	components::{
		connect4_board::Connect4Board, login::LoginPage, navbar::NavBar,
		toot_and_otto_board::TootAndOttoBoard,
	},
	switch::{AppRoute, AppRouter, PublicUrlSwitch},
};
use yew::prelude::*;

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
		let get_route = AppRouter::render(|switch: PublicUrlSwitch| match switch.route() {
			AppRoute::Connect4 => html! {<Connect4Board />},
			AppRoute::Login => html! {<LoginPage/>},
			AppRoute::SignUp => html! {<Connect4Board />},
			AppRoute::TootAndOtto => html! {<TootAndOttoBoard />},
			AppRoute::Home => html! {<TootAndOttoBoard/>},
		});

		html! {
			<div class="app">
				<NavBar />
				<AppRouter render=get_route />
			</div>
		}
	}
}
