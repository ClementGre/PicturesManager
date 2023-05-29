use yew::suspense::Suspense;
use yew::{function_component, html, Html, Properties};

pub use pm_common::gallery_cache::PictureCache;

use crate::mainpane::picture_thumb::PictureThumb;

#[derive(Properties, PartialEq)]
pub struct PicturesListProps {
    pub pics: Vec<String>,
    pub dirs: Vec<String>,
}
#[allow(non_snake_case)]
#[function_component]
pub fn PicturesList(props: &PicturesListProps) -> Html {
    let fallback = html! {
        <li class="loading">
        </li>
    };

    html! {
        <>
            <ul class="pictures-list">
                {

                    props.dirs.iter().map(|path| {
                        html! {
                            <li>{"Directory: "}{path}</li>
                        }
                    }).collect::<Html>()
                }
                {

                    props.pics.iter().map(|id| {
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
