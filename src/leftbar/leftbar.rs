use yew::{Properties, Children, function_component, Html, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn LeftBar() -> Html {
    html! {
        <section class="sidebar leftbar">
            <h2>{"Left"}</h2>
        </section>
    }
}


