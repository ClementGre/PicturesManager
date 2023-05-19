use crate::mainpane::pictures_list::PicturesList;
use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn MainPane() -> Html {
    html! {
        <section class="mainpane">
            <PicturesList />
        </section>
    }
}
