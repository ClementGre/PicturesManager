use yew::suspense::Suspense;
use yew::{function_component, html, Html, Properties};
use yewdux::{use_selector, Dispatch};

use crate::app::{Context, MainPaneDisplayType};
use crate::mainpane::dir_thumb::DirThumb;
use crate::mainpane::picture_thumb::PictureThumb;

#[derive(Properties, PartialEq)]
pub struct PicturesListProps {
    pub root_dir: Vec<String>,
}

#[allow(non_snake_case)]
#[function_component]
pub fn PicturesList(props: &PicturesListProps) -> Html {
    let context_dispatch = Dispatch::<Context>::global();
    let pictures_ids = (*use_selector(|context: &Context| context.main_pane_pictures.clone())).clone();
    let subdirectories = (*use_selector(|context: &Context| context.main_pane_dirs.clone())).clone();

    let to_carousel_cb = context_dispatch.reduce_mut_callback_with(move |ctx, i: usize| {
        ctx.main_pane_content = MainPaneDisplayType::PictureAndCarousel;
        ctx.main_pane_selected_index = Some(i);
    });

    html! {
        <>
            <ul class="pictures-list">
                <Suspense fallback={html!{<></>}}>
                    {
                        subdirectories.iter().map(|dir| {
                            html! {
                                <>
                                    {""} // Without this, the order might not be persistent while loading.
                                    <DirThumb key={dir.clone()} root_dir={props.root_dir.clone()} dir={dir.clone()} />
                                </>
                            }
                        }).collect::<Html>()
                    }
                    {
                    pictures_ids.iter().enumerate().map(|(i, id)| {
                        html! {
                            <>
                                {""} // Without this, the order might not be persistent while loading.
                                <PictureThumb key={id.clone()} id={id.clone()} index={i} to_carousel_cb={to_carousel_cb.clone()}/>
                            </>
                        }
                    }).collect::<Html>()
                }
                </Suspense>
            </ul>
        </>
    }
}
