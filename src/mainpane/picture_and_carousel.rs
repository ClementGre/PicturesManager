use yew::{function_component, html, Html, Properties};

use crate::mainpane::full_picture::FullPicture;
use crate::mainpane::picture_carousel::PictureCarousel;

#[derive(Properties, PartialEq)]
pub struct PictureAndCarouselProps {
    pub id: String,
    pub left_ids: Vec<String>,
    pub right_ids: Vec<String>,
}

#[allow(non_snake_case)]
#[function_component]
pub fn PictureAndCarousel(props: &PictureAndCarouselProps) -> Html {
    html! {
        <div class="picture-and-carousel">
            <FullPicture id={props.id.clone()} />
            <div class="carousel-container">
                <div class="carousel-overflow">
                    <div class="carousel">
                        <ul>
                            {
                                props.left_ids.iter().map(|id| {
                                    html! {
                                        <PictureCarousel key={id.clone()} id={id.clone()} />
                                    }
                                }).collect::<Html>()
                            }
                            </ul>
                            <ul class="selected">
                                <PictureCarousel key={props.id.clone()} id={props.id.clone()} />
                            </ul>
                            <ul>
                            {
                                props.right_ids.iter().map(|id| {
                                    html! {
                                        <PictureCarousel key={id.clone()} id={id.clone()} />
                                    }
                                }).collect::<Html>()
                            }
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
