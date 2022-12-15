use yew::{Properties, Children, function_component, Html, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn RightBar() -> Html {
    html! {
        <section class="sidebar rightbar">
            <h2>{"Right"}</h2>
        </section>
    }
}


