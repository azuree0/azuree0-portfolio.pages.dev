pub mod api;
pub mod app;
pub mod components;
pub mod models;
pub mod scene;

pub use app::App;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    let element = document
        .get_element_by_id("app")
        .expect("app element not found");
    yew::Renderer::<App>::with_root(element).render();
}
