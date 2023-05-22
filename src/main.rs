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

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<AppLoader>::new().render();
}

#[function_component]
pub fn AppLoader() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let fallback = html! {"Loading..."};
    html! {
        <>
            <Suspense {fallback}>
                <App/>
            </Suspense>
        </>
    }
}
