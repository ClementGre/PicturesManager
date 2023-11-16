use yew::{classes, function_component, html, Children, Html, Properties};
use yew_icons::{Icon, IconId};
use yewdux::prelude::{use_selector, Dispatch};

use pm_common::gallery::GalleryData;

use crate::leftbar::files_tree::FilesTree;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[allow(non_snake_case)]
#[function_component]
pub fn LeftBar() -> Html {
    let selected_tab = use_selector(|data: &GalleryData| data.current_left_tab.clone());
    let data_dispatch = Dispatch::<GalleryData>::new();

    html! {
        <section class="sidebar leftbar">
            <div class="tabs-header">
                // Filesystem view
                <button class={classes!(if *selected_tab == 0 { Some("selected") } else { None })}
                     onclick={data_dispatch.reduce_mut_callback(|data| data.current_left_tab = 0)}>
                    <Icon icon_id={IconId::LucideFolderClosed}/>
                </button>
                // Custom hierarchies view
                <button class={classes!(if *selected_tab == 1 { Some("selected") } else { None })}
                    onclick={data_dispatch.reduce_mut_callback(|data| data.current_left_tab = 1)}>
                    <Icon icon_id={IconId::OcticonsListOrdered16}/>
                </button>
                // Fast filtering view
                <button class={classes!(if *selected_tab == 2 { Some("selected") } else { None })}
                    onclick={data_dispatch.reduce_mut_callback(|data| data.current_left_tab = 2)}>
                    <Icon icon_id={IconId::LucideFilter}/>
                </button>
                // Clusters list view
                <button class={classes!(if *selected_tab == 3 { Some("selected") } else { None })}
                    onclick={data_dispatch.reduce_mut_callback(|data| data.current_left_tab = 3)}>
                    <Icon icon_id={IconId::HeroiconsOutlineRectangleGroup}/>
                </button>
            </div>
            {
                if *selected_tab == 0 {
                    html! {
                        <div class="content">
                            <FilesTree />
                        </div>
                    }
                }else if *selected_tab == 1 {
                    html! {
                        <div class="content">
                            {"Custom Hierarchies view"}
                        </div>
                    }
                }else if *selected_tab == 2 {
                    html! {
                        <div class="content">
                            {"Fast filtering view"}
                        </div>
                    }
                }else{
                    html! {
                        <div class="content">
                            {"Clusters list view"}
                        </div>
                    }
                }
            }
        </section>
    }
}
