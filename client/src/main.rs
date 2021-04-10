#![recursion_limit = "512"]

mod app;
mod switch;
mod components {
    pub mod connect4_board;
    pub mod navbar;
    pub mod toot_and_otto_board;
}
mod connect4 {
    pub mod connect4;
    pub mod easy_cpu;
    pub mod hard_cpu;
    pub mod medium_cpu;
}
mod toot_and_otto {
    pub mod toot_and_otto;
}

fn main() {
    yew::start_app::<app::App>();
}
