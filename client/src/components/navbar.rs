use crate::app::AppRoute;
use yew::prelude::*;
use yew_router::prelude::RouterButton;

pub struct NavBar {}

pub enum Msg {}

impl Component for NavBar {
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
			<div class="navbar">
				<h4>{"Board Games"}</h4>
				<div class="navbar__links">
					<RouterButton<AppRoute> route=AppRoute::Login> {"LOG IN"} </RouterButton<AppRoute>>
					<RouterButton<AppRoute> route=AppRoute::Login> {"LOG IN"} </RouterButton<AppRoute>>
				</div>
			</div>
		}
	}
}
