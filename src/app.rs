use futures::stream::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};
use tauri_sys::event::{listen, once};
use tauri_sys::os::{self, OsKind};
use tauri_sys::path::home_dir;
use tauri_sys::window::{self, current_window};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::{use_future, use_future_with_deps};
use yew_hooks::use_is_first_mount;
use yewdux::prelude::use_store;
use yewdux::store::Store;

use pm_common::app_data::{Settings, Theme};
use pm_common::gallery::{GalleryData, GallerySettings};

use crate::header::header::Header;
use crate::leftbar::leftbar::LeftBar;
use crate::mainpane::mainpane::MainPane;
use crate::rightbar::rightbar::RightBar;
use crate::utils::translator::Translator;
use crate::utils::utils::{cmd_async, cmd_async_get};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StaticContext {
    pub macos: bool,
    pub windows: bool,
    pub window_label: String,
    pub home_dir: String,
}

#[derive(Clone, Debug, Default, PartialEq, Store)]
pub enum MainPaneDisplayType {
    // Root dir path as vec, Pictures Ids, Dirs names
    FilesTabPicturesAndDirs(Vec<String>, Vec<String>, Vec<String>),
    #[default]
    None,
}
#[derive(Clone, Debug, Default, PartialEq, Store)]
pub struct MainPaneDimensions {
    pub width: i32,
    pub height: i32,
    pub scroll_top: i32,
    pub scroll_bottom: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Store)]
pub struct Context {
    pub theme: Theme,
    pub gallery_path: String,
    pub main_pane_content: MainPaneDisplayType,
    pub main_pane_dimesions: MainPaneDimensions,
}

#[derive(Serialize, Deserialize)]
pub struct GalleryDataContainer {
    pub data: GalleryData,
}
#[derive(Serialize, Deserialize)]
pub struct GallerySettingsContainer {
    pub settings: GallerySettings,
}

#[allow(non_snake_case)]
#[function_component]
pub fn App() -> HtmlResult {
    /******************************/
    /******* StaticContext ********/
    /******************************/
    let os = use_future(|| async { os::kind().await.unwrap_or(OsKind::Linux) })?;
    let home_dir = use_future(|| async { home_dir().await.expect("No home directory!") })?;
    let static_context = use_memo(
        |_| StaticContext {
            macos: *os == OsKind::Darwin,
            windows: *os == OsKind::WindowsNT,
            window_label: current_window().label(),
            home_dir: home_dir.to_string_lossy().to_string(),
        },
        (),
    );

    /******************************/
    /********** Settings **********/
    /******************************/
    let (settings, settings_dispatch) = use_store::<Settings>();
    let settings_future = use_future(|| async { cmd_async_get::<Settings>("get_settings").await })?;
    if use_is_first_mount() {
        settings_dispatch.set(settings_future.clone());
        let settings_dispatch = settings_dispatch.clone();
        spawn_local(async move {
            let mut events = listen::<Settings>("settings-changed").await.unwrap();
            while let Some(e) = events.next().await {
                settings_dispatch.set(e.payload);
                info!("Settings changed")
            }
        });
    }

    /******************************/
    /********** Context ***********/
    /******************************/
    let (context, context_dispatch) = use_store::<Context>();

    let gallery_path_future = use_future(|| async { cmd_async_get::<String>("get_gallery_path").await })?;
    let os_theme_future = use_future(|| async { current_window().theme().await.unwrap() })?;

    let os_theme = use_state(|| os_theme_future.clone());

    if use_is_first_mount() {
        context_dispatch.reduce_mut(|context| {
            context.gallery_path = gallery_path_future.clone();
        });

        let os_theme = os_theme.clone();
        spawn_local(async move {
            let mut events = listen::<window::Theme>("tauri://theme-changed").await.unwrap();
            while let Some(e) = events.next().await {
                os_theme.set(e.payload);
            }
        });
    }

    // Context theme updated with settings and os_theme
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

    /******************************/
    /******** Gallery Data ********/
    /******************************/

    let gallery_data_future = use_future(|| async { cmd_async_get::<GalleryData>("get_gallery_data").await })?;
    let gallery_settings_future = use_future(|| async { cmd_async_get::<GallerySettings>("get_gallery_settings").await })?;
    let (gallery_data, gallery_data_dispatch) = use_store::<GalleryData>();
    let (gallery_settings, gallery_settings_dispatch) = use_store::<GallerySettings>();

    let close_app = use_state(|| false);
    if *close_app {
        spawn_local(async move {
            info!("🚩 Received close request from frontend, sending back gallery data and settings, then closing window");

            cmd_async::<GalleryDataContainer, ()>(
                "set_gallery_data",
                &GalleryDataContainer {
                    data: (*gallery_data).clone(),
                },
            )
            .await;
            cmd_async::<GallerySettingsContainer, ()>(
                "set_gallery_settings",
                &GallerySettingsContainer {
                    settings: (*gallery_settings).clone(),
                },
            )
            .await;

            current_window().close().await.unwrap();
        });
        return Ok(html! {});
    }

    if use_is_first_mount() {
        gallery_data_dispatch.set((*gallery_data_future).clone());
        gallery_settings_dispatch.set((*gallery_settings_future).clone());
        spawn_local(async move {
            let _ = once::<()>("tauri://close-requested").await.unwrap();
            close_app.set(true);
        });
    }

    Ok(html! {
        <>
            <ContextProvider<StaticContext> context={(*static_context).clone()}>
                <Header class={if context.theme == Theme::Light { "th-light" } else { "th-dark" }}/>
                <main class={if context.theme == Theme::Light { "th-light" } else { "th-dark" }}>
                    <LeftBar/>
                    <MainPane/>
                    <RightBar/>
                </main>
            </ContextProvider<StaticContext>>
        </>
    })
}
