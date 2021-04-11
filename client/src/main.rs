#![recursion_limit = "1024"]

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
    pub mod piece_color;
}

mod toot_and_otto {
    pub mod piece_letter;
    pub mod player;
    pub mod toot_and_otto;
}

mod types {
    pub mod opponent;
}

use wasm_logger;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}
