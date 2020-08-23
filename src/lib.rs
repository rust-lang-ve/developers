#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod api;
mod app;
mod components;
mod models;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}
