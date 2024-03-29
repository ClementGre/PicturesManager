use futures::stream::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};
use tauri_sys::event::{listen, once};
use tauri_sys::os::{self, OsKind};
use tauri_sys::path::home_dir;
use tauri_sys::window::{self, current_window};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::{use_future, use_future_with};
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
    pub protocol: &'static str,
    pub window_label: String,
    pub home_dir: String,
}

#[derive(Clone, Debug, Default, PartialEq, Store)]
pub enum MainPaneDisplayType {
    PicturesAndDirs(Vec<String>), // Root dir path as vec
    PictureAndCarousel,
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
    pub main_pane_pictures: Vec<String>,
    pub main_pane_dirs: Vec<String>,
    pub main_pane_selected_index: Option<usize>,
    pub main_pane_selected_indices: Vec<usize>,
    pub main_pane_dimensions: MainPaneDimensions,
}

impl Context {
    pub fn select_index(&mut self, i: usize, shift: bool, ctrl: bool) {
        if let Some(i_from) = self.main_pane_selected_index {
            if shift {
                self.select_range(i_from, i, ctrl);
            } else if ctrl {
                self.select_single(i, true);
            } else {
                self.select_single(i, false);
            }
        } else {
            self.main_pane_selected_index = Some(i);
            self.main_pane_selected_indices = vec![i];
        }
    }
    fn select_range(&mut self, i_from: usize, i_to: usize, add: bool) {
        let mut new_selected_indices = Vec::new();
        if i_from > i_to {
            for i in i_to..=i_from {
                new_selected_indices.push(i);
            }
        } else {
            for i in i_from..=i_to {
                new_selected_indices.push(i);
            }
        }
        if add {
            self.main_pane_selected_indices.extend(new_selected_indices);
            self.main_pane_selected_indices.sort();
            self.main_pane_selected_indices.dedup();
            self.main_pane_selected_index = Some(i_to);
        } else {
            self.main_pane_selected_indices = new_selected_indices;
        }
    }
    fn select_single(&mut self, i: usize, add: bool) {
        if add {
            if self.main_pane_selected_indices.contains(&i) {
                self.main_pane_selected_indices.retain(|&j| j != i);
                if self.main_pane_selected_index == Some(i) {
                    self.main_pane_selected_index = self.main_pane_selected_indices.last().copied();
                }
            } else {
                self.main_pane_selected_indices.push(i);
                self.main_pane_selected_index = Some(i);
            }
        } else {
            self.main_pane_selected_index = Some(i);
            self.main_pane_selected_indices = vec![i];
        }
    }

    pub fn get_selected_picture_ids(&self) -> Vec<String> {
        if let Some(i) = self.main_pane_selected_index {
            if self.main_pane_selected_indices.len() > 1 {
                return self
                    .main_pane_selected_indices
                    .iter()
                    .map(|i| self.main_pane_pictures[*i].clone())
                    .collect();
            }
            return vec![self.main_pane_pictures[i].clone()];
        }
        return Vec::new();
    }
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
    let static_context = use_memo((), |_| StaticContext {
        macos: *os == OsKind::Darwin,
        windows: *os == OsKind::WindowsNT,
        protocol: if *os != OsKind::WindowsNT {
            "reqimg://localhost"
        } else {
            "https://reqimg.localhost"
        },
        window_label: current_window().label(),
        home_dir: home_dir.to_string_lossy().to_string(),
    });

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

        spawn_local(async move {
            let mut events = listen::<String>("contex_menu_tree_item_files").await.unwrap();
            while let Some(e) = events.next().await {
                if e.window_label == Some(current_window().label()) {
                    info!("🚩 Received context menu event {:?}", e.payload);
                }
            }
        });
    }

    // Context theme updated with settings and os_theme
    {
        let settings = settings.clone();
        use_effect_with((settings, os_theme), move |(settings, os_theme)| {
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
        });
    }

    /******************************/
    /********* Translator *********/
    /******************************/
    let (_, translator_dispatch) = use_store::<Translator>();
    let language = {
        let settings = settings.clone();
        use_memo(settings, move |settings| settings.language.clone())
    };
    let _ = {
        let language = language.clone();
        // Acts as an async callback that updates with language change
        use_future_with(language.clone(), |language| async move {
            let translator = Translator::new((**language).clone()).await;
            translator_dispatch.set(translator);
        })?
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
