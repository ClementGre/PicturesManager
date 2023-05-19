use yew::{function_component, html, Children, Html, Properties};
use yewdux::prelude::use_selector;

use crate::app::Context;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn LeftBar() -> Html {
    let selected_tab = use_selector(|context: &Context| context.left_tab.clone());
    
    html! {
        <section class="sidebar leftbar">
            <div class="tabs-header">

            </div>
            <div class="content">

            </div>
        </section>
    }
}
