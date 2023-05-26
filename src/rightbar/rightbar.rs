use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[allow(non_snake_case)]
#[function_component]
pub fn RightBar() -> Html {
    html! {
        <section class="sidebar rightbar">
            <h2>{"Right"}</h2>
        </section>
    }
}
