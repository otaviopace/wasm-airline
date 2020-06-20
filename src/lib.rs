mod app;

use wasm_bindgen::prelude::*;
use yew::prelude::App;


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::Airline>::new().mount_to_body();
}