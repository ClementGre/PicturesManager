use crate::utils::logger::*;
use crate::utils::utils::{cmd_async, cmd_async_get};
use futures::stream::StreamExt;
use pm_common::data_structs::Theme;
use serde::{Deserialize, Serialize};
use tauri_sys::event::listen;
use tauri_sys::os::{self, OsKind};
use tauri_sys::window;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future;

use crate::header::header::Header;
use crate::leftbar::leftbar::LeftBar;
use crate::mainpane::mainpane::MainPane;
use crate::rightbar::rightbar::RightBar;
use crate::utils::translator::Translator;

#[derive(Clone, Debug, PartialEq)]
pub struct Context {
    pub macos: bool,
    pub windows: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GreetArgs<'a> {
    pub name: &'a str,
}

#[function_component]
pub fn App() -> HtmlResult {
    let lang = use_future(|| async { cmd_async_get::<String>("get_language").await })?;
    tr((*lang).as_str());

    let theme = use_future(|| async { cmd_async_get::<Theme>("get_theme_or_os").await })?;
    let theme = use_state(|| *theme);
    let os = use_future(|| async { os::kind().await.unwrap_or(OsKind::Linux) })?;

    /* Context définition */
    let context = {
        use_state(|| Context {
            macos: *os == OsKind::Darwin,
            windows: *os == OsKind::WindowsNT,
        })
    };

    let translator = Translator::new("fr-FR".parse().expect("Invalid language identifier"));

    /* OS theme sync */
    spawn_local({
        let theme = theme.clone();
        async move {
            let mut events = listen::<tauri_sys::window::Theme>("tauri://theme-changed").await.unwrap();
            while let Some(e) = events.next().await {
                if cmd_async::<(), bool>("is_system_theme", &()).await {
                    theme.set(if e.payload == window::Theme::Light { Theme::LIGHT } else { Theme::DARK });
                }
            }
        }
    });

    Ok(html! {
        <>
            <ContextProvider<Context> context={(*context).clone()}>
                <Header class={if *theme == Theme::LIGHT { "th-light" } else { "th-dark" }}/>
                <main class="light">
                    <LeftBar/>
                    <MainPane/>
                    <RightBar/>
                </main>

            </ContextProvider<Context>>
        </>
    })
}
