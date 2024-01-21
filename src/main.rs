use yew::suspense::Suspense;
use yew::{function_component, html, Html};

use crate::app::App;
use crate::utils::logger::init_backend_logger;

mod app;
mod components;
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
    init_backend_logger().expect("Failed to initialize backend logger");
    yew::Renderer::<AppLoader>::new().render();
}

#[function_component(AppLoader)]
pub fn app_loader() -> Html {
    let fallback = html! {
        <>
            <header class="th-light"/>
            <main class="th-light">
                <section class="sidebar leftbar"/>
                <section class="mainpane"/>
                <section class="side-bar rightbar"/>
            </main>
        </>
    };
    html! {
        <>
            <Suspense {fallback}>
                <App/>
            </Suspense>
        </>
    }
}
