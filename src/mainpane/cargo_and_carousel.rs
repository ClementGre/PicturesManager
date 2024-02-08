use yew::{function_component, html, Html, Properties};

use crate::mainpane::full_picture::FullPicture;

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
            <div class="carousel">
                <ul>
                    <li>
                    </li>
                    <li>
                    </li>
                    <li>
                    </li>
                    <li>
                    </li>
                    <li>
                    </li>
                    <li class="selected">
                    </li>
                    <li>
                    </li>
                    <li>
                    </li>
                    <li>
                    </li>
                    <li>
                    </li>
                    <li>
                    </li>
                    <li>
                    </li>
                </ul>
                // {
                //     props.left_ids.iter().map(|id| {
                //         html! {
                //             <li>
                //                 <img src={format!("/get-thumbnail?id={}", id)} />
                //             </li>
                //         }
                //     }).collect::<Html>()
                // }
                // {
                //     props.right_ids.iter().map(|id| {
                //         html! {
                //             <li>
                //                 <img src={format!("/get-thumbnail?id={}", id)} />
                //             </li>
                //         }
                //     }).collect::<Html>()
                // }
            </div>
        </div>
    }
}
