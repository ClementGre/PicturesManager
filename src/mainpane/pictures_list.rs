use yew::suspense::Suspense;
use yew::{function_component, html, Html, Properties};
use yewdux::Dispatch;

use crate::app::{Context, MainPaneDisplayType};
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
    let context_dispatch = Dispatch::<Context>::global();
    let select_picture = {
        let pics = props.pics.clone();
        context_dispatch.reduce_mut_callback_with(move |ctx, i: usize| {
            ctx.main_pane_content = MainPaneDisplayType::PictureAndCarousel(pics.clone(), i);
        })
    };

    html! {
        <>
            <ul class="pictures-list">
                <Suspense fallback={html!{<></>}}>
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
                        props.pics.iter().enumerate().map(|(i, id)| {
                            let select_picture = select_picture.clone();
                            html! {
                                <>
                                    {""} // Without this, the order might not be persistent while loading.
                                    <PictureThumb key={id.clone()} id={id.clone()} select_callback={move |_| {select_picture.emit(i);}}/>
                                </>
                            }
                        }).collect::<Html>()
                    }
                </Suspense>
            </ul>
        </>
    }
}
