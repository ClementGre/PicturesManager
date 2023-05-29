use yew::{function_component, html, Html, Properties};
use yew_icons::{Icon, IconId};
use yewdux::dispatch::Dispatch;

use pm_common::gallery::GalleryData;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub path: Vec<String>,
}
#[allow(non_snake_case)]
#[function_component]
pub fn DirThumb(props: &Props) -> Html {
    let data_dispatch = Dispatch::<GalleryData>::new();

    let onclick = {
        let path = props.path.clone();
        data_dispatch.reduce_mut_callback(move |data| {
            data.files_tab_selected_dir = path.clone();
        })
    };

    html! {
        <li style={format!("flex-basis: {}px; flex-grow: {};", 140, 140)} onclick={onclick}>
            <div class="thumb dir-thumb" style={format!("aspect-ratio: 1;")}>
                <Icon icon_id={IconId::FontAwesomeSolidFolderOpen} />
                <p>{props.path.last().unwrap_or(&String::default())}</p>
            </div>
        </li>
    }
}
