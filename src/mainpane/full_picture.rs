use log::warn;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, WheelEvent};
use yew::suspense::Suspense;
use yew::{function_component, html, suspense::use_future_with, use_context, use_node_ref, HtmlResult, NodeRef, Properties};
use yew_hooks::{use_is_first_mount, use_size, use_update};
use yewdux::{use_selector, Dispatch};

use pm_common::gallery::GalleryData;

use crate::{app::StaticContext, utils::utils::cmd_async};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetImageArgs {
    pub(crate) id: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}
#[allow(non_snake_case)]
#[function_component]
pub fn FullPicture(props: &Props) -> HtmlResult {
    let dimensions = use_future_with(props.id.clone(), |id| async move {
        cmd_async::<GetImageArgs, Option<(u32, u32)>>("get_image_dimensions", &GetImageArgs { id: id.to_string() }).await
    })?;

    let ref_container = use_node_ref();

    // Force component to re-render when the container size change.
    let (container_width, container_height) = use_size(ref_container.clone());

    if let Some((width, height)) = *dimensions {
        let fallback = html! {
            <div class="full-image loading" ref={ref_container.clone()}>
            </div>
        };

        return Ok(html! {
            <Suspense fallback={fallback}>
                <div class="full-image" ref={ref_container.clone()}>
                    <FullPictureImage id={props.id.clone()} {width} {height} {ref_container} {container_width} {container_height}/>
                </div>
            </Suspense>
        });
    }

    warn!("No cached dimensions for image {}", props.id);
    return Ok(html! { <div></div> });
}

#[derive(Properties, PartialEq)]
pub struct ImageProps {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub ref_container: NodeRef,
    pub container_width: u32,
    pub container_height: u32,
}
#[allow(non_snake_case)]
#[function_component]
fn FullPictureImage(props: &ImageProps) -> HtmlResult {
    let static_ctx = use_context::<StaticContext>().unwrap();
    let zoom = use_selector(|data: &GalleryData| data.zoom_carousel.clone());
    let data_dispatch = Dispatch::<GalleryData>::global();

    let update = use_update();
    let is_first_mount = use_is_first_mount();
    let ref_image = use_node_ref();

    if is_first_mount {
        update();
        // Force a first empty render to initialize node ref
        return Ok(html! { <div class="image" ref={ref_image}></div> });
    }

    let container = props.ref_container.cast::<HtmlElement>().unwrap();
    let image = ref_image.cast::<HtmlElement>().unwrap();

    let cont_w = container.client_width() as f64;
    let cont_h = container.client_height() as f64;
    let img_w = image.client_width() as f64;
    let img_h = image.client_height() as f64;

    let mut left = 0;
    let mut top = 0;
    if cont_w > img_w * *zoom {
        left = ((cont_w - img_w * *zoom) / 2.0) as i32;
    }
    if cont_h > img_h * *zoom {
        top = ((cont_h - img_h * *zoom) / 2.0) as i32;
    }

    let onwheel = {
        let container = container.clone();
        data_dispatch.reduce_mut_callback_with(move |data, e: WheelEvent| {
            if e.ctrl_key() {
                e.prevent_default();
                let zoom = *zoom.clone();
                data.zoom_carousel = (zoom * (1.0 - zoom / 40.0)).max(1.0).min(5.0);
                let factor = data.zoom_carousel / zoom;

                let rect = container.get_bounding_client_rect();
                let x = e.client_x() as f64 - rect.left();
                let y = e.client_y() as f64 - rect.top();

                container.set_scroll_left((x * (factor - 1.0) + factor * container.scroll_left() as f64) as i32);
                container.set_scroll_top((y * (factor - 1.0) + factor * container.scroll_top() as f64) as i32);
            }
        })
    };

    let protocol = if !static_ctx.windows {
        "reqimg://localhost"
    } else {
        "https://reqimg.localhost"
    };
    Ok(html! {
        <div class="image" ref={ref_image} {onwheel}
            style={format!("background-image: url({}/get-image?id={}&window={}); aspect-ratio: {}/{}; scale: {}; left: {}px; top: {}px;",
            protocol, props.id, static_ctx.window_label, props.width, props.height, *zoom, left, top)}>
        </div>
    })
}
