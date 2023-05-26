use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Children, Html, Properties};
use yewdux::prelude::use_store;

use pm_common::gallery_cache::PathsCache;

use crate::app::{Context, MainPaneDisplayType};
use crate::mainpane::mainpane::CacheContext;

#[allow(non_snake_case)]
#[function_component]
pub fn FilesTree() -> Html {
    let (cache, _) = use_store::<CacheContext>();

    html! {
        <ul>
            {
                cache.paths_cache.children.iter().map(|path_cache| {
                    html! {
                        <DirTree path_cache={path_cache.clone()} />
                    }
                }).collect::<Html>()
            }
        </ul>
    }
}

#[derive(Properties, PartialEq)]
struct DirTreeProps {
    pub path_cache: PathsCache,
}
#[allow(non_snake_case)]
#[function_component]
fn DirTree(props: &DirTreeProps) -> Html {
    let (_, ctx_dispatch) = use_store::<Context>();
    let on_click = {
        let pictures = props.path_cache.pictures.clone();
        Callback::from(move |_| {
            ctx_dispatch.reduce_mut(|ctx| {
                ctx.main_pane_content = MainPaneDisplayType::Pictures(pictures.clone());
            });
        })
    };

    html! {
        <li>
            <p onclick={on_click}>{&props.path_cache.dir_name}</p>
            <ul>
                {
                    props.path_cache.children.iter().map(|path_cache| {
                        html! {
                            <DirTree path_cache={path_cache.clone()} />
                        }
                    }).collect::<Html>()
                }
            </ul>
        </li>
    }
}
