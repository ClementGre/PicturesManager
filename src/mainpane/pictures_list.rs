use yew::suspense::Suspense;
use yew::{function_component, html, Children, Html, Properties};
use yewdux::prelude::use_store;

pub use pm_common::gallery_cache::PictureCache;

use crate::mainpane::mainpane::CacheContext;
use crate::mainpane::picture_thumb::PictureThumb;

#[derive(Properties, PartialEq)]
pub struct PicturesListProps {
    pub pics: Vec<String>,
    pub dirs: Vec<String>,
}
#[allow(non_snake_case)]
#[function_component]
pub fn PicturesList(props: &PicturesListProps) -> Html {
    let (cache, _) = use_store::<CacheContext>();

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
