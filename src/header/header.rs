use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn Header() -> Html {
    html! {
        <header data-tauri-drag-region="true">
            <p>{"Testttt"}</p>
        </header>
    }
}