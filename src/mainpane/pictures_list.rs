use yew::suspense::Suspense;
use yew::{function_component, html, Html, Properties};

use crate::mainpane::dir_thumb::DirThumb;
use crate::mainpane::picture_thumb::PictureThumb;

#[derive(Properties, PartialEq)]
pub struct PicturesListProps {
    pub root_dir: Vec<String>,
    pub pics: Vec<String>,
    pub dirs: Vec<String>,
}
#[allow(non_snake_case)]
#[function_component]
pub fn PicturesList(props: &PicturesListProps) -> Html {
    let fallback = html! {
        <></>
    };

    html! {
        <>
            <ul class="pictures-list">
                <Suspense fallback={fallback.clone()}>
                    {
                        props.dirs.iter().map(|dir| {
                            html! {
                                <>
                                    {""} // Without this, the order might not be persistent while loading.
                                    <DirThumb key={dir.clone()} root_dir={props.root_dir.clone()} dir={dir.clone()} />
                                </>
                            }
                        }).collect::<Html>()
                    }
                    {
                        props.pics.iter().map(|id| {
                            html! {
                                <>
                                    {""} // Without this, the order might not be persistent while loading.
                                    <PictureThumb key={id.clone()} id={id.clone()} />
                                </>
                            }
                        }).collect::<Html>()
                    }
                </Suspense>
            </ul>
        </>
    }
}
