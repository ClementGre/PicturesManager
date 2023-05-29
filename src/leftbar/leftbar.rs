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
    let selected_tab = use_selector(|data: &GalleryData| data.last_left_tab.clone());
    let data_dispatch = Dispatch::<GalleryData>::new();

    html! {
        <section class="sidebar leftbar">
            <div class="tabs-header">
                // System filesystem view
                <button class={classes!(if *selected_tab == 0 { Some("selected") } else { None })}
                     onclick={data_dispatch.reduce_mut_callback(|data| data.last_left_tab = 0)}>
                    <Icon icon_id={IconId::OcticonsFileDirectoryOpenFill16}/>
                </button>
                // Tags view
                <button class={classes!(if *selected_tab == 1 { Some("selected") } else { None })}
                    onclick={data_dispatch.reduce_mut_callback(|data| data.last_left_tab = 1)}>
                    <Icon icon_id={IconId::FontAwesomeSolidTag}/>
                </button>
                // Date view
                <button class={classes!(if *selected_tab == 2 { Some("selected") } else { None })}
                    onclick={data_dispatch.reduce_mut_callback(|data| data.last_left_tab = 2)}>
                    <Icon icon_id={IconId::BootstrapCalendarDateFill}/>
                </button>
                // Locations view
                <button class={classes!(if *selected_tab == 3 { Some("selected") } else { None })}
                    onclick={data_dispatch.reduce_mut_callback(|data| data.last_left_tab = 3)}>
                    <Icon icon_id={IconId::FontAwesomeSolidMapLocationDot}/>
                </button>
                // Custom views
                <button class={classes!(if *selected_tab == 4 { Some("selected") } else { None })}
                    onclick={data_dispatch.reduce_mut_callback(|data| data.last_left_tab = 4)}>
                    <Icon icon_id={IconId::FontAwesomeSolidBinoculars}/>
                </button>
            </div>
            <div class="content" style={if *selected_tab == 0 {""} else {"display: none;"}}>
                <FilesTree />
            </div>
            <div class="content" style={if *selected_tab == 1 {""} else {"display: none;"}}>
                {"Tags view"}
            </div>
            <div class="content" style={if *selected_tab == 2 {""} else {"display: none;"}}>
               {"Date view"}
            </div>
            <div class="content" style={if *selected_tab == 3 {""} else {"display: none;"}}>
               {"Locations view"}
            </div>
            <div class="content" style={if *selected_tab == 4 {""} else {"display: none;"}}>
               {"Custom view"}
            </div>
        </section>
    }
}
