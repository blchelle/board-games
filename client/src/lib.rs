#![recursion_limit = "1024"]
mod app;
mod components {
    pub mod connect4_board;
    pub mod navbar;
}
mod connect4 {
    pub mod connect4;
    pub mod easy_cpu;
    pub mod hard_cpu;
    pub mod medium_cpu;
}

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}
