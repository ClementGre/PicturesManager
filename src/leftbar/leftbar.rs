use fluent::fluent_args;
use pm_common::app_data::Settings;
use url::Url;
use web_sys::window;
use yew::{function_component, html, Children, Html, Properties, AttrValue};
use yewdux::prelude::use_store;

use crate::{app::Context, utils::{translator::Translator}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn LeftBar() -> Html {
    let url = window().unwrap().location().href().unwrap();
    let gallery = Url::parse(&url).unwrap().query_pairs().find(|(key, _)| key == "p").unwrap().1.to_string();


    let (context, _) = use_store::<Context>();
    let (settings, _) = use_store::<Settings>();
    let (t, _) = use_store::<Translator>();
    
    html! {
        <section class="sidebar leftbar">
            <h2>{"Url"}</h2>
            <p>{format!("Url: {}", url)}</p>
            <p>{format!("Gallery: {}", gallery)}</p>
            <h2>{"Context"}</h2>
            {
                Html::from_html_unchecked(AttrValue::from({format!("<p>{:#?}</p>", context).replace("\n", "<br>").replace("  ", "&nbsp;&nbsp;")}))
            }
            <h2>{"Settings"}</h2>
            {
                Html::from_html_unchecked(AttrValue::from({format!("<p>{:#?}</p>", settings).replace("\n", "<br>").replace("  ", "&nbsp;&nbsp;")}))
            }
            <h2>{"Translator"}</h2>
            <p>{format!("Test tr : {}", t.tr("hello"))}</p>
            <p>{format!("Test tra : {}", t.tra("test", &fluent_args!["name" => "Clément", "nombre" => 4]))}</p>
        </section>
    }
}
