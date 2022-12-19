use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{Html, function_component, html};
use yew::suspense::Suspense;
use crate::app::App;

mod app;
mod header;
mod leftbar;
mod rightbar;
mod mainpane;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

fn main() {
    yew::Renderer::<AppLoader>::new().render();
}

#[function_component]
pub fn AppLoader() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let fallback = html! {};

    html! {
        <>
            <Suspense {fallback}>
                <App/>
            </Suspense>
        </>
    }
}