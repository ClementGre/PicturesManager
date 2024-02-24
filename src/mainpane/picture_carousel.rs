use log::warn;
use web_sys::HtmlElement;
use yew::{
    Callback, classes, function_component, html, HtmlResult, Properties, suspense::use_future_with, use_context, use_effect, use_node_ref,
    use_state_eq,
};
use yew::suspense::Suspense;
use yew_hooks::use_size;
use yewdux::Dispatch;

use crate::{app::StaticContext, utils::utils::cmd_async};
use crate::app::{Context, MainPaneDisplayType};
use crate::mainpane::full_picture::GetImageArgs;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub index: usize,
    pub selected: bool,
    pub set_offset: Callback<(i32, i32)>, // li left offset, li width
}

#[allow(non_snake_case)]
#[function_component]
pub fn PictureCarousel(props: &Props) -> HtmlResult {
    let dimensions = use_future_with(props.id.clone(), |id| async move {
        cmd_async::<GetImageArgs, Option<(u32, u32)>>("get_image_dimensions", &GetImageArgs { id: id.to_string() }).await
    })?;
    // let main_pane_dimensions = use_selector(|ctx: &Context| ctx.main_pane_dimensions.clone());

    let li_ref = use_node_ref();
    let offset = use_state_eq(|| 0i32);
    let carousel_height = 60;

    // Switch to another picture of the carousel on click
    let context_dispatch = Dispatch::<Context>::global();

    let set_offset = {
        let offset = offset.clone();
        let set_offset = props.set_offset.clone();
        let selected = props.selected.clone();
        let li_ref = li_ref.clone();
        Callback::from(move |mut new_offset: i32| {
            if new_offset != 0 {
                offset.set(new_offset);
            } else {
                new_offset = *offset;
            }
            if selected {
                let mut width = 0;
                if let Some(li) = li_ref.cast::<HtmlElement>() {
                    width = li.offset_width();
                }
                set_offset.emit((new_offset, width));
            }
        })
    };

    let onclick = {
        let index = props.index.clone();
        let offset = offset.clone();
        let set_offset = set_offset.clone();
        context_dispatch.reduce_mut_callback(move |data| {
            if let MainPaneDisplayType::PictureAndCarousel(pictures_ids, _) = data.main_pane_content.clone() {
                data.main_pane_content = MainPaneDisplayType::PictureAndCarousel(pictures_ids, index);
                set_offset.emit(*offset); // Will force offset to be updated to the value of this li.
            }
        })
    };

    if let Some((width, height)) = *dimensions {
        let fallback = html! {
            <li class={classes!(Some("loading"), if props.selected { Some("selected") } else { None })} onclick={onclick.clone()}>
                <div class="image" style={format!("width: {}px; height: {}px;", carousel_height*width/height, carousel_height)}/>
            </li>
        };
        return Ok(html! {
            <Suspense fallback={fallback}>
                <li class={classes!(if props.selected { Some("selected") } else { None })} onclick={onclick} ref={li_ref}>
                    <PictureCarouselImage id={props.id.clone()} width={carousel_height*width/height} height={carousel_height} {set_offset}/>
                </li>
            </Suspense>
        });
    }

    warn!("No cached dimensions for image {}", props.id);
    return Ok(html! { <li></li> });
}

#[derive(Properties, PartialEq)]
pub struct ImageProps {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub set_offset: Callback<i32>,
}

#[allow(non_snake_case)]
#[function_component]
fn PictureCarouselImage(props: &ImageProps) -> HtmlResult {
    let static_ctx = use_context::<StaticContext>().unwrap();

    let has_thumb = use_future_with(props.id.clone(), |id| async move {
        cmd_async::<GetImageArgs, bool>("gen_image_thumbnail", &GetImageArgs { id: id.to_string() }).await
    })?;

    if !*has_thumb {
        warn!("No thumb for {}", props.id);
        return Ok(html! {});
    }

    let ref_img = use_node_ref();
    let _ = use_size(ref_img.clone());

    use_effect({
        let set_offset = props.set_offset.clone();
        let ref_img = ref_img.clone();
        move || {
            if let Some(img_div) = ref_img.cast::<HtmlElement>() {
                set_offset.emit(img_div.offset_left());
            }
        }
    });

    Ok(html! {
        <div class="image" ref={ref_img.clone()}
            style={format!("background-image: url({}/get-thumbnail?id={}&window={}); width: {}px; height: {}px;",
            static_ctx.protocol, props.id, static_ctx.window_label, props.width, props.height)}>
        </div>
    })
}
