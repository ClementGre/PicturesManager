use crate::invoke_async;
use crate::utils::logger::*;
use futures::stream::StreamExt;
use pm_common::data_structs::Theme;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use tauri_sys::event::listen;
use tauri_sys::os::{self, OsKind};
use tauri_sys::tauri::invoke;
use tauri_sys::window;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future;

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
    let lang = use_future(|| async { invoke::<(), String>("get_language", &()).await.unwrap() })?;
    tr((*lang).as_str());

    let theme = use_future(|| async { invoke("get_theme_or_os", &()).await.unwrap_or(Theme::LIGHT) })?;
    let theme = use_state(|| *theme);
    let os = use_future(|| async { os::kind().await.unwrap_or(OsKind::Linux) })?;

    /* Context définition */
    let context = {
        use_state(|| Context {
            macos: *os == OsKind::Darwin,
            windows: *os == OsKind::WindowsNT,
        })
    };

    /* Greet example command */
    let greet = {
        Callback::from(move |_| {
            spawn_local(async {
                let new_msg = invoke_async("greet", to_value(&GreetArgs { name: &*"test" }).unwrap()).await;
                info(new_msg.as_string().unwrap().as_str());
            });
        })
    };

    /* OS theme sync */
    spawn_local({
        let theme = theme.clone();
        async move {
            let mut events = listen::<tauri_sys::window::Theme>("tauri://theme-changed").await.unwrap();
            while let Some(e) = events.next().await {
                if invoke::<(), bool>("is_system_theme", &()).await.unwrap() {
                    theme.set(if e.payload == window::Theme::Light { Theme::LIGHT } else { Theme::DARK });
                }
            }
        }
    });

    Ok(html! {
        <>
            <ContextProvider<Context> context={(*context).clone()}>
                <Header class={if *theme == Theme::LIGHT { "th-light" } else { "th-dark" }}/>
                <button type="button" onclick={greet}>{"Greet"}</button>
                <main class="light">
                    <LeftBar/>
                    <MainPane/>
                    <RightBar/>
                </main>

            </ContextProvider<Context>>
        </>
    })
}
