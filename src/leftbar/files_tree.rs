use std::rc::Rc;

use yew::{function_component, html, Html, use_callback, use_state_eq};
use yewdux::prelude::{Dispatch, use_selector, use_store};

use pm_common::gallery::GalleryData;
use pm_common::gallery_cache::PathsCache;

use crate::app::{Context, MainPaneDisplayType};
use crate::components::treeitem::TreeItemData;
use crate::components::treeview::TreeView;
use crate::mainpane::mainpane::CacheContext;

fn to_tree_item_props(path_cache: &PathsCache) -> TreeItemData {
    TreeItemData {
        id: path_cache.dir_name.clone(), // dir name is only unique than it can be used as id
        name: path_cache.dir_name.clone(),
        children: Rc::new(path_cache.children.iter().map(|child| Rc::new(to_tree_item_props(child))).collect()),
    }
}
fn vec_to_path_cache<'a>(root_path_cache: &'a PathsCache, path: &[String]) -> Option<&'a PathsCache> {
    if path.len() == 0 {
        return None;
    }
    root_path_cache.children.iter().find(|child| child.dir_name == path[0]).and_then(|child| {
        if path.len() == 1 {
            Some(child)
        } else {
            vec_to_path_cache(child, &path[1..])
        }
    })
}
fn make_valid_path(root_path_cache: &PathsCache, path: &Vec<String>) -> Vec<String> {
    let mut selected_path_cache = root_path_cache;
    let mut made_path = Vec::default();
    for dir in path {
        let result = selected_path_cache.children.iter().find(|child| child.dir_name == *dir);
        if let Some(path_cache) = result {
            selected_path_cache = path_cache;
            made_path.push(dir.clone());
        } else {
            break;
        }
    }
    made_path
}

#[allow(non_snake_case)]
#[function_component]
pub fn FilesTree() -> Html {
    let (cache, _) = use_store::<CacheContext>();
    //let current_left_tab = use_selector(|data: &GalleryData| data.current_left_tab.clone());
    let selected_dir = use_selector(|data: &GalleryData| data.files_tab_selected_dir.clone());
    let data_dispatch = Dispatch::<GalleryData>::global();
    let (ctx, ctx_dispatch) = use_store::<Context>();

    let last_selected_path = use_state_eq(|| Vec::default());

    // Updating selected_dir when treeview selected_path changes.
    let selected_changed = {
        let data_dispatch = data_dispatch.clone();
        let last_selected_path = last_selected_path.clone();
        let ctx = ctx.clone();
        use_callback((), move |path: Vec<String>, _| {
            if let MainPaneDisplayType::PicturesAndDirs(_, _, _) = ctx.main_pane_content {
            } else {
                // Force MainPaneDisplayType to be updated.
                last_selected_path.set(Vec::default());
            }
            data_dispatch.reduce_mut(move |data| {
                data.files_tab_selected_dir = path;
            });
        })
    };

    // Updating main_pane content when selected_dir changes.
    if *last_selected_path != *selected_dir && cache.is_loaded {
        let path = make_valid_path(&cache.paths_cache, &selected_dir);
        last_selected_path.set(path.clone());
        if path != *selected_dir {
            data_dispatch.reduce_mut(|data| {
                data.files_tab_selected_dir = path.clone();
            });
        }

        let path_cache = vec_to_path_cache(&cache.paths_cache, &selected_dir);
        if let Some(path_cache) = path_cache {
            let pictures = path_cache.pictures.clone();
            let dirs: Vec<String> = path_cache.children.iter().map(|child| child.dir_name.clone()).collect();

            ctx_dispatch.reduce_mut(|ctx| {
                ctx.main_pane_content = MainPaneDisplayType::PicturesAndDirs((*selected_dir).clone(), pictures, dirs);
            });
        }
    }

    let items = cache
        .paths_cache
        .children
        .iter()
        .map(|path_cache| Rc::new(to_tree_item_props(path_cache)))
        .collect::<Vec<Rc<TreeItemData>>>();

    html! {
        <TreeView
            items={items}
            selected_changed={selected_changed}
            selected_path={Rc::clone(&selected_dir)}
            />
    }
}
