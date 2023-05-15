use crate::header::header::Header;
use crate::leftbar::leftbar::LeftBar;
use crate::mainpane::mainpane::MainPane;
use crate::rightbar::rightbar::RightBar;
use crate::utils::translator::Translator;
use crate::utils::utils::cmd_async_get;
use futures::stream::StreamExt;
use pm_common::app_data::{Settings, Theme};
use tauri_sys::event::listen;
use tauri_sys::os::{self, OsKind};
use tauri_sys::window::{self, current_window};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::{use_future, use_future_with_deps};
use yewdux::prelude::use_store;
use yewdux::store::Store;

#[derive(Clone, Debug, Default, PartialEq, Store)]
pub struct Context {
    pub macos: bool,
    pub windows: bool,
    pub theme: Theme,
}

#[function_component]
pub fn App() -> HtmlResult {
    /******************************/
    /********** Settings **********/
    /******************************/
    let (settings, settings_dispatch) = use_store::<Settings>();
    let settings_future = use_future(|| async { cmd_async_get::<Settings>("get_settings").await })?;
    {
        let settings_dispatch = settings_dispatch.clone();
        use_effect_with_deps(
            move |_| {
                // Initial setup
                settings_dispatch.set(settings_future.clone());
            },
            (),
        );
    }

    /******************************/
    /**** Context: Os & Theme *****/
    /******************************/
    let (context, context_dispatch) = use_store::<Context>();
    // OS
    let _ = {
        let context_dispatch = context_dispatch.clone();
        use_future(|| async move {
            let os = os::kind().await.unwrap_or(OsKind::Linux);
            context_dispatch.reduce_mut(|context| {
                context.macos = os == OsKind::Darwin;
                context.windows = os == OsKind::WindowsNT;
            });
        })?
    };

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
    {
        let settings = settings.clone();
        use_effect_with_deps(
            move |(settings, os_theme)| {
                context_dispatch.reduce_mut(|context| {
                    context.theme = if settings.theme == Theme::System {
                        if **os_theme == window::Theme::Light {
                            Theme::Light
                        } else {
                            Theme::Dark
                        }
                    } else {
                        settings.theme
                    }
                })
            },
            (settings, os_theme),
        );
    }

    /******************************/
    /********* Translator *********/
    /******************************/
    let (_, translator_dispach) = use_store::<Translator>();
    let language = {
        let settings = settings.clone();
        use_memo(move |settings| settings.language.clone(), settings)
    };
    let _ = {
        let language = language.clone();
        // Acts as an async callback that updates with language change
        use_future_with_deps(
            |language| async move {
                let translator = Translator::new((**language).clone()).await;
                translator_dispach.set(translator);
            },
            language.clone(),
        )?
    };

    Ok(html! {
        <>
             <ContextProvider<Context> context={(*context).clone()}>
                    <Header class={if context.theme == Theme::Light { "th-light" } else { "th-dark" }}/>
                    <main class="light">
                        <LeftBar/>
                        <MainPane/>
                        <RightBar/>
                    </main>
             </ContextProvider<Context>>
        </>
    })
}
