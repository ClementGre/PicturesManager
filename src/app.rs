use crate::header::header::Header;
use crate::leftbar::leftbar::LeftBar;
use crate::mainpane::mainpane::MainPane;
use crate::rightbar::rightbar::RightBar;
use crate::utils::translator::Translator;
use crate::utils::utils::cmd_async_get;
use futures::stream::StreamExt;
use pm_common::app_data::{Settings, Theme};
use std::rc::Rc;
use tauri_sys::event::listen;
use tauri_sys::os::{self, OsKind};
use tauri_sys::window::{self, current_window};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::{use_future, use_future_with_deps};

#[derive(Clone, Debug, PartialEq)]
pub struct Context {
    pub macos: bool,
    pub windows: bool,
    pub theme: Theme,
}

#[function_component]
pub fn App() -> HtmlResult {
    // OS
    let os = use_future(|| async { os::kind().await.unwrap_or(OsKind::Linux) })?;

    // Settings got with use_future saved in state
    let settings_future = use_future(|| async { cmd_async_get::<Settings>("get_settings").await })?;
    let settings = use_state(|| settings_future.clone());

    // OS theme got with use_future and might be updated with event listener
    let os_theme_future = use_future(|| async { current_window().theme().await.unwrap() })?;
    let os_theme = use_state(|| os_theme_future.clone());
    spawn_local({
        let os_theme = os_theme.clone();
        async move {
            let mut events = listen::<tauri_sys::window::Theme>("tauri://theme-changed").await.unwrap();
            while let Some(e) = events.next().await {
                os_theme.set(e.payload);
            }
        }
    });

    // Theme memo calculated based on settings and os_theme
    let theme = {
        let settings = settings.clone();
        let os_theme = os_theme.clone();
        use_memo(
            move |(settings, os_theme)| {
                if settings.theme == Theme::System {
                    if **os_theme == window::Theme::Light {
                        Theme::Light
                    } else {
                        Theme::Dark
                    }
                } else {
                    settings.theme
                }
            },
            (settings, os_theme),
        )
    };

    // Language and translations
    let language = {
        let settings = settings.clone();
        use_memo(move |settings| settings.language.clone(), settings)
    };
    let translator = {
        let language = language.clone();
        use_future_with_deps(
            |language| async move {
                let translator = Translator::new((**language).clone()).await;
                translator
            },
            language.clone(),
        )?
    };

    // Context définition
    let context = {
        let os = os.clone();
        let theme = theme.clone();
        use_memo(
            |(os, theme)| Context {
                macos: *os == OsKind::Darwin,
                windows: *os == OsKind::WindowsNT,
                theme: *theme.clone(),
            },
            (os, theme),
        )
    };

    // Change language tests
    let change_language_fr = {
        let settings = settings.clone();
        Callback::from(move |_| {
            let mut new_settings = (*settings).clone();
            new_settings.language = Some("fr".to_string());
            settings.set(new_settings);
        })
    };
    let change_language_en = {
        let settings = settings.clone();
        Callback::from(move |_| {
            let mut new_settings = (*settings).clone();
            new_settings.language = Some("en".to_string());
            settings.set(new_settings);
        })
    };

    Ok(html! {
        <>
             <ContextProvider<Context> context={(*context).clone()}>
                <ContextProvider<Settings> context={(*settings).clone()}>
                    <ContextProvider<Translator> context={(*translator).clone()}>
                        <Header class={if *theme == Theme::Light { "th-light" } else { "th-dark" }}/>
                        <main class="light">
                            <LeftBar/>
                            <MainPane/>
                            <RightBar/>
                            <button onclick={change_language_fr}>{"FR"}</button>
                            <button onclick={change_language_en}>{"EN"}</button>
                        </main>
                    </ContextProvider<Translator>>
                </ContextProvider<Settings>>
             </ContextProvider<Context>>
        </>
    })
}
