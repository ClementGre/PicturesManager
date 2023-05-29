use std::collections::HashMap;

use futures::stream::StreamExt;
use tauri_sys::event::listen;
use yew::platform::spawn_local;
use yew::suspense::use_future;
use yew::{function_component, html, Children, Html, Properties};
use yewdux::prelude::{use_selector, use_store};
use yewdux::store::Store;

use pm_common::gallery_cache::{PathsCache, PictureCache};

use crate::app::Context;
use crate::app::MainPaneDisplayType::FilesTabPicturesAndDirs;
use crate::mainpane::pictures_list::PicturesList;
use crate::utils::utils::cmd_async_get;

#[derive(Clone, Debug, Default, PartialEq, Store)]
pub struct CacheContext {
    pub datas_cache: HashMap<String, PictureCache>,
    pub paths_cache: PathsCache,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[allow(non_snake_case)]
#[function_component]
pub fn MainPane() -> Html {
    let (_, cache_dispatch) = use_store::<CacheContext>();
    let _ = {
        let cache_dispatch = cache_dispatch.clone();
        use_future(|| async move {
            cache_dispatch.set(CacheContext {
                datas_cache: cmd_async_get::<HashMap<String, PictureCache>>("get_gallery_datas_cache").await,
                paths_cache: cmd_async_get::<PathsCache>("get_gallery_paths_cache").await,
            });
        })
    };
    spawn_local({
        let cache_dispatch = cache_dispatch.clone();
        async move {
            let mut events = listen::<(HashMap<String, PictureCache>, PathsCache)>("gallery-cache-changed")
                .await
                .unwrap();
            while let Some(e) = events.next().await {
                cache_dispatch.set(CacheContext {
                    datas_cache: e.payload.0,
                    paths_cache: e.payload.1,
                });
            }
        }
    });

    let content = use_selector(|context: &Context| context.main_pane_content.clone());

    html! {
        <section class="mainpane">
            {
                if let FilesTabPicturesAndDirs(pics, dirs) = (*content).clone() {
                    html! {
                        <PicturesList pics={pics} dirs={dirs}/>
                    }
                }else{
                    html!{
                        <div class="empty">
                            <p>{"Nothing to display"}</p>
                        </div>
                }
                }

            }
        </section>
    }
}
