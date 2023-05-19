use std::collections::HashMap;
use pm_common::gallery_cache::{PathsCache, PictureCache};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, suspense::use_future, use_context, Callback, Children, Html, Properties};
use yewdux::{prelude::use_store, store::Store};

use crate::{app::StaticContext, utils::{utils::cmd_async_get, logger::info}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[derive(Clone, Debug, Default, PartialEq, Store)]
pub struct CacheContext {
    pub datas_cache: HashMap<String, PictureCache>,
    pub paths_cache: PathsCache,
}

#[function_component]
pub fn PicturesList() -> Html {
    let static_ctx = use_context::<StaticContext>().unwrap();
    let (cache, cache_dispatch) = use_store::<CacheContext>();

    let _ = {
        let cache_dispatch = cache_dispatch.clone();
        use_future(|| async move {
            cache_dispatch.set(CacheContext {
                datas_cache: cmd_async_get::<HashMap<String, PictureCache>>("get_gallery_datas_cache").await,
                paths_cache: cmd_async_get::<PathsCache>("get_gallery_paths_cache").await,
            });
        })
    };

    let update_data = Callback::from(move |_| {
        let cache_dispatch = cache_dispatch.clone();
        spawn_local(async move {
            let (datas_cache, paths_cache) = cmd_async_get::<(HashMap<String, PictureCache>, PathsCache)>("update_gallery_cache").await;
            cache_dispatch.set(CacheContext { datas_cache, paths_cache });
        });
    });
    let mut count = 0;
    info("rendering pictures list");
    html! {
        <>
            <button onclick={update_data}>{"Update"}</button>
            <ul class="pictures-list">
                {

                    cache.datas_cache.iter().map(|(id, _)| {
                        count += 1;
                        if count > 1 {
                            return html! {

                            }
                        }
                        html! {
                            <li style={format!("background-image: url(reqimg://id/?id={}&window={}); min-width: 100px;", id, static_ctx.window_label)}>
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </>
    }
}
