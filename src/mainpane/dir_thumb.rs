use yew::{function_component, html, Html, Properties};
use yew_icons::{Icon, IconId};
use yewdux::dispatch::Dispatch;

use pm_common::gallery::GalleryData;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub root_dir: Vec<String>,
    pub dir: String,
}
#[allow(non_snake_case)]
#[function_component]
pub fn DirThumb(props: &Props) -> Html {
    let data_dispatch = Dispatch::<GalleryData>::global();

    let onclick = {
        let mut path = props.root_dir.clone();
        path.push(props.dir.clone());
        data_dispatch.reduce_mut_callback(move |data| {
            data.files_tab_selected_dir = path.clone();
        })
    };

    html! {
        <li style={format!("flex-basis: {}px; flex-grow: {};", 140, 140)} onclick={onclick}>
            <div class="thumb dir-thumb">
                <Icon icon_id={IconId::FontAwesomeSolidFolderOpen} />
                <p>{props.dir.clone()}</p>
            </div>
        </li>
    }
}
