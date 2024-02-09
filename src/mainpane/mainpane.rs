use std::collections::HashMap;

use futures::stream::StreamExt;
use log::debug;
use tauri_sys::event::listen;
use web_sys::HtmlElement;
use yew::platform::spawn_local;
use yew::suspense::use_future;
use yew::{function_component, html, use_node_ref, use_state, Callback, Children, Html, Properties, Suspense};
use yew_hooks::use_size;
use yewdux::prelude::{use_selector, use_store, Dispatch};
use yewdux::store::Store;

use pm_common::gallery_cache::{PathsCache, PictureCache};

use crate::app::MainPaneDisplayType;
use crate::app::{Context, MainPaneDimensions};
use crate::mainpane::picture_and_carousel::PictureAndCarousel;
use crate::mainpane::pictures_list::PicturesList;
use crate::utils::utils::cmd_async_get;

#[derive(Clone, Debug, Default, PartialEq, Store)]
pub struct CacheContext {
    pub is_loaded: bool,
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
    let context_dispatch = Dispatch::<Context>::global();

    // CacheContext loading and updating system
    let (_, cache_dispatch) = use_store::<CacheContext>();
    let _ = {
        let cache_dispatch = cache_dispatch.clone();
        use_future(|| async move {
            cache_dispatch.set(CacheContext {
                is_loaded: true,
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
                    is_loaded: true,
                    datas_cache: e.payload.0,
                    paths_cache: e.payload.1,
                });
            }
        }
    });

    // Syncing main pane dimensions to Context
    let node = use_node_ref();
    let on_scroll_zone_changed = {
        let node = node.clone();
        context_dispatch.reduce_mut_callback_with(move |context, _: yew::Event| {
            let target = node.cast::<HtmlElement>().unwrap();
            context.main_pane_dimensions = MainPaneDimensions {
                height: target.client_height(),
                width: target.client_width(),
                scroll_top: target.scroll_top(),
                scroll_bottom: target.scroll_top() + target.client_height(),
            };
        })
    };
    let size_state = use_state(|| (0u32, 0u32));
    let size = use_size(node.clone());
    if *size_state != size {
        size_state.set(size);
        on_scroll_zone_changed.emit(yew::Event::new("onscroll").unwrap());
    }

    let onkeypress = {
        Callback::from(move |e| {
            debug!("Keypres {:?}", e);
        })
    };

    let content = (*use_selector(|context: &Context| context.main_pane_content.clone())).clone();

    html! {
        <section ref={node} class="mainpane" onscroll={on_scroll_zone_changed.clone()} onkeypress={onkeypress}>
            <Suspense fallback={html! { <div class="empty"></div> }}>
            {
                if let MainPaneDisplayType::PicturesAndDirs(root_dir, pics, dirs) = content{
                    html! {
                        <PicturesList {root_dir} {pics} {dirs}/>
                    }
                }else if let MainPaneDisplayType::PictureAndCarousel(id, left_ids, right_ids) = content {
                    html! {
                        <PictureAndCarousel {id} {left_ids} {right_ids}/>
                    }
                }
                else{
                    html!{
                        <div class="empty">
                            <p>{"Nothing to display"}</p>
                        </div>
                    }
                }
            }
            </Suspense>
        </section>

    }
}
