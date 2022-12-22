use crate::{invoke_async};
use crate::utils::logger::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

use crate::header::header::Header;
use crate::leftbar::leftbar::LeftBar;
use crate::mainpane::mainpane::MainPane;
use crate::rightbar::rightbar::RightBar;


#[derive(Clone, Debug, PartialEq)]
pub struct Context {
    pub macos: bool,
    pub windows: bool,
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component]
pub fn App() -> HtmlResult {

    let lang = window().unwrap().navigator().languages().to_vec().iter().map(|val| val.as_string().unwrap()).collect::<Vec<String>>();
    info(lang.join(",").as_str());

    //let os = *use_future(|| async { invoke_async("get_os", JsValue::default()).await.as_f64().unwrap() as u16 })?;

    let os = window().unwrap().navigator().app_version().unwrap();
    let context = use_state(|| Context { macos: os.contains("Mac"), windows: os.contains("Win") });

    /*let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let new_msg =
                        invoke("greet", to_value(&GreetArgs { name: &*name }).unwrap()).await;
                    log(&new_msg.as_string().unwrap());
                    greet_msg.set(new_msg.as_string().unwrap());
                });

                || {}
            },
            name2,
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |_| {
            name.set(greet_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value());
        })
    };

    <div class="row">
                    <a href="https://tauri.app" target="_blank">
                        <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                    </a>
                    <a href="https://yew.rs" target="_blank">
                        <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                    </a>
                </div>

                <p>{"Click on the Tauri and Yew logos to learn more."}</p>

                <p>
                    {"Recommended IDE setup: "}
                    <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
                    {" + "}
                    <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
                    {" + "}
                    <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
                </p>

                <div class="row">
                    <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                    <button type="button" onclick={greet}>{"Greet"}</button>
                </div>

                <p><b>{ &*greet_msg }</b></p>*/

    let event = Callback::from(move |_| {
        spawn_local(async move {
            let new_msg = invoke_async("greet", to_value(&GreetArgs { name: &*"test" }).unwrap()).await;
            info(new_msg.as_string().unwrap().as_str());
        });
    });


    Ok(html! {
        <>
            <ContextProvider<Context> context={(*context).clone()}>
                <Header/>
                <button type="button" onclick={event}>{"Greet"}</button>
                <main>
                    <LeftBar/>
                    <MainPane/>
                    <RightBar/>
                </main>

            </ContextProvider<Context>>
        </>
    })
}
