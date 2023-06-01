use std::rc::Rc;

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
    let selected_dir = use_selector(|data: &GalleryData| data.files_tab_selected_dir.clone());
    let current_left_tab = use_selector(|data: &GalleryData| data.current_left_tab.clone());

    html! {
        <ul class="files-tree root">
            {
                cache.paths_cache.children.iter().enumerate().map(|(i, path_cache)| {
                    html! {
                        <DirTree key={i} parents={Vec::default()} path_cache={Rc::new(path_cache.clone())}
                            selected_dir={selected_dir.clone()} is_current_tab={*current_left_tab == 0}/>
                    }
                }).collect::<Html>()
            }
        </ul>
    }
}

#[derive(Properties, PartialEq)]
struct DirTreeProps {
    pub parents: Vec<String>,
    pub path_cache: Rc<PathsCache>,
    pub selected_dir: Rc<Vec<String>>,
    pub is_current_tab: bool,
}
#[allow(non_snake_case)]
#[function_component]
fn DirTree(props: &DirTreeProps) -> Html {
    let ctx_dispatch = Dispatch::<Context>::new();
    let data_dispatch = Dispatch::<GalleryData>::new();

    let mut parents = props.parents.clone();
    parents.push(props.path_cache.dir_name.clone());
    let is_selected = *props.selected_dir == parents;

    let is_open = use_state_eq(|| false);
    let was_selected = use_state(|| false); // Selected on last render

    // Display the directory in the main pane
    let show_content = {
        let pictures = props.path_cache.pictures.clone();
        let dirs = props.path_cache.children.clone();
        let parents = parents.clone();
        use_callback(
            move |_, _| {
                let dirs: Vec<String> = dirs.iter().map(|child| child.dir_name.clone()).collect();
                ctx_dispatch.reduce_mut(|ctx| {
                    ctx.main_pane_content = MainPaneDisplayType::FilesTabPicturesAndDirs(parents.clone(), pictures.clone(), dirs.clone());
                });
            },
            (),
        )
    };

    // Open if a children is selected
    if !*is_open && !is_selected && props.selected_dir.starts_with(&parents) {
        is_open.set(true);
    }
    // Show content if just selected
    if is_selected && props.is_current_tab && !*was_selected {
        show_content.emit(());
        was_selected.set(true);
    }

    let toggle_open = {
        let is_open = is_open.clone();
        let parents = parents.clone();
        let selected_dir = props.selected_dir.clone();
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
                                        <DirTree key={i} parents={parents.clone()} path_cache={path_cache.clone()}
                                            selected_dir={props.selected_dir.clone()} is_current_tab={props.is_current_tab}/>
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
