use yew::{Properties, Children, function_component, Html, html, Callback};

use crate::utils::utils::cmd;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn MainPane() -> Html {

    let update_data = Callback::from(move |_| {
        cmd("update_gallery_cache");
    });

    html! {
        <section class="mainpane">
            <button onclick={update_data}>{"Update"}</button>
            <h2>{"MainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMainMain"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}<br/>{"Main"}</h2>
            <img src="reqimg://reqimg/?path=/Users/clement/Pictures/Icones/banner.png&test=te" alt=""/>
        </section>
    }
}