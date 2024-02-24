use std::cmp::max;

use log::info;
use web_sys::Event;
use yew::{Callback, function_component, html, Html, Properties, use_mut_ref, use_node_ref, use_state_eq};
use yew_hooks::{use_size, use_update};

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

    let _ = use_size(ref_ul.clone());
    let padding: i32 = 3;
    let pad_left = use_state_eq(|| 0);
    let pad_right = use_state_eq(|| 0);

    let set_offset = {
        let ref_ul = ref_ul.clone();
        let pad_left = pad_left.clone();
        let pad_right = pad_right.clone();
        Callback::from(move |(offset, width)| {
            if let Some(ul) = ref_ul.cast::<web_sys::HtmlElement>() {
                let bounds = ul.get_bounding_client_rect();
                info!("- Updating carousel offset, offset: {}px, width: {}px", offset, width);
                info!("    Scroll Width: {}px, bounds width: {}px", ul.scroll_width(), bounds.width());
                let mut scroll: i32 = (offset - (*pad_left - padding) + width / 2) - bounds.width() as i32 / 2i32;
                let max_scroll = max(0i32, ul.scroll_width() - (*pad_right + *pad_left - 2 * padding) - bounds.width() as i32);
                let mut new_pad_left = 0;
                let mut new_pad_right = 0;
                if scroll < 0 {
                    new_pad_left = -scroll;
                    scroll = 0;
                } else if scroll > max_scroll - padding {
                    new_pad_right = scroll - max_scroll;
                    scroll = ul.scroll_width() - bounds.width() as i32;
                }
                info!("    New scroll: {}, pad_left: {}, pad_right: {}", scroll, new_pad_left, new_pad_right);
                pad_left.set(new_pad_left + padding);
                pad_right.set(new_pad_right + padding);
                ul.set_scroll_left(scroll);
            }
        })
    };

    html! {
        <div class="picture-and-carousel">
            <FullPicture id={props.pictures_ids[props.selected_index].clone()} />
            <div class="carousel-container">
                <div class="carousel-overflow">
                    <div class="carousel">
                        <ul ref={ref_ul} {onscroll} style={format!("padding-left: {}px; padding-right: {}px;", *pad_left, *pad_right)}>
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
