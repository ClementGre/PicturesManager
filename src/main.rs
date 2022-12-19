use yew::{Html, function_component, html};
use yew::suspense::Suspense;
use crate::app::App;

mod app;
mod header;
mod leftbar;
mod rightbar;
mod mainpane;
mod utils;

fn main() {
    yew::Renderer::<AppLoader>::new().render();
}

#[function_component]
pub fn AppLoader() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <>
            <Suspense {fallback}>
                <App/>
            </Suspense>
        </>
    }
}