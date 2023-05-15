use fluent::fluent_args;
use pm_common::app_data::Settings;
use url::Url;
use web_sys::window;
use yew::{function_component, html, use_context, Children, Html, Properties};
use yewdux::prelude::{use_selector, use_store};

use crate::{app::Context, utils::translator::Translator};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn LeftBar() -> Html {
    let url = window().unwrap().location().href().unwrap();
    let gallery = Url::parse(&url).unwrap().query_pairs().find(|(key, _)| key == "p").unwrap().1.to_string();

    let context = use_context::<Context>().unwrap();

    let (settings, _) = use_store::<Settings>();
    let settings_language = use_selector(|settings: &Settings| settings.language.clone());

    let (tr, _) = use_store::<Translator>();
    
    html! {
        <section class="sidebar leftbar">
            <h2>{"Url"}</h2>
            <p>{format!("Url: {}", url)}</p>
            <p>{format!("Gallery: {}", gallery)}</p>
            <h2>{"Context"}</h2>
            <p>{format!("Current theme: {:?}", context.theme)}</p>
            <p>{format!("Is_macos, is_windows: {:?}, {:?}", context.macos, context.windows)}</p>
            <h2>{"Settings"}</h2>
            <p>{format!("Settings theme: {:?}", settings.theme)}</p>
            <p>{format!("Settings language: {:?}", settings_language)}</p>
            <h2>{"Translator"}</h2>
            <p>{format!("Test tr : {}", tr.tr("hello"))}</p>
            <p>{format!("Test tra : {}", tr.tra("test", &fluent_args!["name" => "Clément", "nombre" => 4]))}</p>
        </section>
    }
}
