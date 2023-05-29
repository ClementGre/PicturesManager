use yew::{classes, function_component, html, use_callback, use_state, use_state_eq, Callback, Html, Properties};
use yew_icons::{Icon, IconId};
use yewdux::prelude::{use_selector, use_store, Dispatch};

use pm_common::gallery::GalleryData;
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
                        <DirTree key={i} parents={Vec::default()} path_cache={path_cache.clone()} />
                    }
                }).collect::<Html>()
            }
        </ul>
    }
}

#[derive(Properties, PartialEq)]
struct DirTreeProps {
    pub parents: Vec<String>,
    pub path_cache: PathsCache,
}
#[allow(non_snake_case)]
#[function_component]
fn DirTree(props: &DirTreeProps) -> Html {
    let (_, ctx_dispatch) = use_store::<Context>();
    let selected_dir = use_selector(|data: &GalleryData| data.files_tab_selected_dir.clone());
    let current_left_tab = use_selector(|data: &GalleryData| data.current_left_tab.clone());
    let data_dispatch = Dispatch::<GalleryData>::new();

    let mut parents = props.parents.clone();
    parents.push(props.path_cache.dir_name.clone());
    let is_selected = *selected_dir == parents;

    let is_open = use_state_eq(|| false);
    let was_selected = use_state(|| false);

    let show_content = {
        let parents = parents.clone();
        let pictures = props.path_cache.pictures.clone();
        let dirs = props.path_cache.children.clone();
        use_callback(
            move |_, _| {
                let dirs: Vec<Vec<String>> = dirs
                    .iter()
                    .map(|child| {
                        let mut parents = parents.clone();
                        parents.push(child.dir_name.clone());
                        parents
                    })
                    .collect();
                ctx_dispatch.reduce_mut(|ctx| {
                    ctx.main_pane_content = MainPaneDisplayType::FilesTabPicturesAndDirs(pictures.clone(), dirs.clone());
                });
            },
            (),
        )
    };

    // Open if a children is selected
    if !*is_open && !is_selected && selected_dir.starts_with(&parents) {
        is_open.set(true);
    }
    // Show content if just selected
    if is_selected && *current_left_tab == 0 && !*was_selected {
        show_content.emit(());
        was_selected.set(true);
    }

    let toggle_open = {
        let is_open = is_open.clone();
        let parents = parents.clone();
        let selected_dir = selected_dir.clone();
        let data_dispatch = data_dispatch.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
            if !is_selected && selected_dir.starts_with(&parents) {
                // Select this dir instead of the children
                data_dispatch.reduce_mut(|data| {
                    data.files_tab_selected_dir = parents.clone();
                });
            }
        })
    };
    let select = {
        let parents = parents.clone();
        Callback::from(move |_| {
            data_dispatch.reduce_mut(|data| {
                data.files_tab_selected_dir = parents.clone();
            });
        })
    };

    // Update was_selected
    if !is_selected && *was_selected {
        was_selected.set(false);
    } else if is_selected && !*was_selected {
        was_selected.set(true);
    }

    html! {
        <li>
            <div class={classes!(if is_selected { Some("selected") } else { None })}>
                {
                    if props.path_cache.children.len() > 0 {
                        html! {
                            <div class={classes!(if *is_open { Some("opened") } else { None })}
                                onclick={toggle_open}>
                                <Icon icon_id={IconId::FontAwesomeSolidAngleRight} />
                            </div>
                        }
                    } else {
                        html! { <div /> }
                    }
                }
                <p onclick={select}>{&props.path_cache.dir_name}</p>
            </div>
            {
                if *is_open && props.path_cache.children.len() > 0 {
                    html! {
                        <ul class="files-tree" style="margin-left: 20px">
                            {
                                props.path_cache.children.iter().enumerate().map(|(i, path_cache)| {
                                    html! {
                                        <DirTree key={i} parents={parents.clone()} path_cache={path_cache.clone()} />
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
