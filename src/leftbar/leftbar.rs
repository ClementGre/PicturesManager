use url::Url;
use web_sys::window;
use yew::{Properties, Children, function_component, Html, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn LeftBar() -> Html {

    let url =  window().unwrap().location().href().unwrap();
    let gallery = Url::parse(&url).unwrap().query_pairs().find(|(key, _)| key == "p").unwrap().1.to_string();

    html! {
        <section class="sidebar leftbar">
            <p>{format!("Url: {}", url)}</p>
            <p>{format!("Gallery: {}", gallery)}</p>
        </section>
    }
}


