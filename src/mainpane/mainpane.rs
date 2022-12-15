use yew::{Properties, Children, function_component, Html, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn MainPane() -> Html {
    html! {
        <section class="mainpane">
            <h2>{"MainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMain"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}</h2>
        </section>
    }
}