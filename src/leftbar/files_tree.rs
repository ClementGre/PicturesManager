use yew::{classes, function_component, html, use_state, Callback, Html, Properties};
use yew_icons::{Icon, IconId};
use yewdux::prelude::use_store;

use pm_common::gallery_cache::PathsCache;

use crate::app::{Context, MainPaneDisplayType};
use crate::mainpane::mainpane::CacheContext;

#[allow(non_snake_case)]
#[function_component]
pub fn FilesTree() -> Html {
    let (cache, _) = use_store::<CacheContext>();

    html! {
        <ul class="files-tree root">
            {
                cache.paths_cache.children.iter().enumerate().map(|(i, path_cache)| {
                    html! {
                        <DirTree key={i} depth={1} path_cache={path_cache.clone()} />
                    }
                }).collect::<Html>()
            }
        </ul>
    }
}

#[derive(Properties, PartialEq)]
struct DirTreeProps {
    pub depth: usize,
    pub path_cache: PathsCache,
}
#[allow(non_snake_case)]
#[function_component]
fn DirTree(props: &DirTreeProps) -> Html {
    let is_open = use_state(|| false);

    let (_, ctx_dispatch) = use_store::<Context>();
    let on_click = {
        let pictures = props.path_cache.pictures.clone();
        Callback::from(move |_| {
            ctx_dispatch.reduce_mut(|ctx| {
                ctx.main_pane_content = MainPaneDisplayType::Pictures(pictures.clone());
            });
        })
    };

    let is_open_clone = is_open.clone();
    html! {
        <li>
            <div>
                {
                    if props.path_cache.children.len() > 0 {
                        html! {
                            <div class={classes!(if *is_open { Some("opened") } else { None })}
                                onclick={move |_| is_open_clone.set(!*is_open_clone)}>
                                <Icon icon_id={IconId::FontAwesomeSolidAngleRight} />
                            </div>
                        }
                    } else {
                        html! { <div /> }
                    }
                }
                <p onclick={on_click}>{&props.path_cache.dir_name}</p>
            </div>
            {
                if *is_open && props.path_cache.children.len() > 0 {
                    html! {
                        <ul class="files-tree" style="margin-left: 20px">
                            {
                                props.path_cache.children.iter().enumerate().map(|(i, path_cache)| {
                                    html! {
                                        <DirTree key={i} depth={props.depth+1} path_cache={path_cache.clone()} />
                                    }
                                }).collect::<Html>()
                            }
                        </ul>
                    }
                } else { html! {} }
            }
        </li>
    }
}
