use std::collections::HashMap;

use wasm_bindgen_futures::spawn_local;
use yew::suspense::Suspense;
use yew::{function_component, html, suspense::use_future, Callback, Children, Html, Properties};
use yewdux::{prelude::use_store, store::Store};

use pm_common::gallery_cache::{PathsCache, PictureCache};

use crate::{mainpane::picture_thumb::PictureThumb, utils::utils::cmd_async_get};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[derive(Clone, Debug, Default, PartialEq, Store)]
pub struct CacheContext {
    pub datas_cache: HashMap<String, PictureCache>,
    pub paths_cache: PathsCache,
}

#[allow(non_snake_case)]
#[function_component]
pub fn PicturesList() -> Html {
    let (cache, _) = use_store::<CacheContext>();

    let fallback = html! {
        <li class="loading">
        </li>
    };

    let mut count = 0;
    html! {
        <>
            <ul class="pictures-list">
                {

                    cache.datas_cache.iter().map(|(id, _)| {
                        count += 1;
                        if count > 50 {
                            return html! {

                            }
                        }
                        html! {
                            <Suspense fallback={fallback.clone()} key={id.clone()}>
                                <PictureThumb id={id.clone()} />
                            </Suspense>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </>
    }
}
