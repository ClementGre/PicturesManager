use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use yew::suspense::Suspense;
use yew::{function_component, html, Html};

use crate::app::App;

mod app;
mod header;
mod leftbar;
mod mainpane;
mod rightbar;
mod utils;

// Cargo.toml
// src
// ├── main.rs
// src-tauri
// ├── Cargo.toml
// ├── src
// │   ├── main.rs
// src-common
// ├── Cargo.toml
// ├── src
// │   ├── main.rs



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = invoke)]
    pub async fn invoke_async(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

fn main() {
    // let bundle = load_locale();

    // let mut errors = vec![];
    // let msg = bundle
    //     .format_value_sync("test", None, &mut errors)
    //     .expect("Message doesn't exist.");

    wasm_logger::init(wasm_logger::Config::default());
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
