use log::info;
use web_sys::Event;
use yew::{Callback, function_component, html, Html, Properties, use_effect_with, use_mut_ref, use_node_ref, use_state, use_state_eq};

use crate::mainpane::full_picture::FullPicture;
use crate::mainpane::picture_carousel::PictureCarousel;

#[derive(Properties, PartialEq)]
pub struct PictureAndCarouselProps {
    pub pictures_ids: Vec<String>,
    pub selected_index: usize,
}

#[allow(non_snake_case)]
#[function_component]
pub fn PictureAndCarousel(props: &PictureAndCarouselProps) -> Html {
    let carousel_scroll = use_mut_ref(|| 0i32);
    let ref_ul = use_node_ref();

    let onscroll = {
        let carousel_scroll = carousel_scroll.clone();
        let ref_ul = ref_ul.clone();
        Callback::from(move |_: Event| {
            if let Some(ref_ul) = ref_ul.cast::<web_sys::HtmlElement>() {
                *carousel_scroll.borrow_mut() = ref_ul.scroll_left();
            }
        })
    };

    // use_effect_with(props.selected_index.clone(), {
    //     let carousel_scroll = carousel_scroll.clone();
    //     let ref_ul = ref_ul.clone();
    //     move |_| {
    //         if let Some(ref_ul) = ref_ul.cast::<web_sys::HtmlElement>() {
    //             ref_ul.set_scroll_left(*carousel_scroll.borrow());
    //             info!(
    //                 "Setting carousel_scroll to {}, width = {}",
    //                 *carousel_scroll.borrow(),
    //                 ref_ul.scroll_width()
    //             );
    //         }
    //         || {}
    //     }
    // });

    let left = use_state_eq(|| 0i32);
    let scroll = use_state_eq(|| 0i32);

    let set_offset = {
        let ref_ul = ref_ul.clone();
        Callback::from(move |offset: i32| {
            if let Some(ref_ul) = ref_ul.cast::<web_sys::HtmlElement>() {
                let bounds = ref_ul.get_bounding_client_rect();
                info!("Setting scroll_left to {}", offset - bounds.width() as i32 / 2i32);
                ref_ul.set_scroll_left(offset - bounds.width() as i32 / 2i32);
            }
        })
    };

    html! {
        <div class="picture-and-carousel">
            <FullPicture id={props.pictures_ids[props.selected_index].clone()} />
            <div class="carousel-container">
                <div class="carousel-overflow">
                    <div class="carousel">
                        <ul ref={ref_ul} {onscroll}>
                            {
                                props.pictures_ids.iter().enumerate().map(|(i, id)| {
                                    html! {
                                        <PictureCarousel key={id.clone()} id={id.clone()} index={i} selected={i == props.selected_index} set_offset={set_offset.clone()}/>
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
