use pm_common::app_data::Settings;
use url::Url;
use web_sys::window;
use yew::{Properties, Children, function_component, Html, html, use_context};

use crate::app::Context;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn LeftBar() -> Html {

    let url =  window().unwrap().location().href().unwrap();
    let gallery = Url::parse(&url).unwrap().query_pairs().find(|(key, _)| key == "p").unwrap().1.to_string();

    let settings = use_context::<Settings>().unwrap();
    let context = use_context::<Context>().unwrap();

    html! {
        <section class="sidebar leftbar">
            <p>{format!("Url: {}", url)}</p>
            <p>{format!("Gallery: {}", gallery)}</p>
            <p>{format!("Settings theme: {:?}", settings.theme)}</p>
            <p>{format!("Settings language: {:?}", settings.language)}</p>
            <p>{format!("Current theme: {:?}", context.theme)}</p>
            <p>{format!("Is_macos, is_windows: {:?}, {:?}", context.macos, context.windows)}</p>
        </section>
    }
}


