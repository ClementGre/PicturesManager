use js_sys::Math::abs;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use web_sys::{HtmlElement, WheelEvent};
use yew::{
    Callback, function_component, html, HtmlResult, NodeRef, Properties, suspense::use_future_with, use_context, use_effect_with, use_node_ref,
    use_state,
};
use yew::suspense::Suspense;
use yew_hooks::{use_is_first_mount, use_size, use_update};

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
    let zoom = use_state(|| 1.0);
    use_effect_with(props.id.clone(), {
        let zoom = zoom.clone();
        move |_| {
            zoom.set(1.0);
            || {}
        }
    });

    let ref_container = use_node_ref();
    let left_s = use_state(|| 0f64);
    let top_s = use_state(|| 0f64);

    // Force component to re-render when the container size change.
    let (container_width, container_height) = use_size(ref_container.clone());

    if let Some((width, height)) = *dimensions {
        let fallback = html! {
            <div class="full-image loading">
            </div>
        };
        let onwheel = {
            let container = ref_container.clone().cast::<HtmlElement>();
            let zoom = zoom.clone();
            let left_s = left_s.clone();
            let top_s = top_s.clone();
            Callback::from(move |e: WheelEvent| {
                if e.ctrl_key() {
                    if let Some(container) = container.clone() {
                        e.prevent_default();

                        let mut left = *left_s;
                        let mut top = *top_s;
                        if abs(*left_s - container.scroll_left() as f64) > 3.0 || abs(*top_s - container.scroll_top() as f64) > 3.0 {
                            left = container.scroll_left() as f64;
                            top = container.scroll_top() as f64;
                            info!("Scrolled manually in full picture pane!");
                        }

                        let old_zoom = *zoom;
                        let new_zoom = (old_zoom * (1.0 - e.delta_y() / 40.0)).max(1.0).min(5.0);
                        let factor = new_zoom / old_zoom;

                        let x = e.client_x() as f64 - container.get_bounding_client_rect().left();
                        let y = e.client_y() as f64 - container.get_bounding_client_rect().top();

                        let zoom_point_x = (x + left) * factor;
                        let zoom_point_y = (y + top) * factor;

                        let scroll_left = (zoom_point_x - x).max(0f64);
                        let scroll_top = (zoom_point_y - y).max(0f64);

                        left_s.set(scroll_left);
                        top_s.set(scroll_top);
                        container.set_scroll_left(scroll_left as i32);
                        container.set_scroll_top(scroll_top as i32);
                        zoom.set(new_zoom);
                    }
                }
            })
        };

        return Ok(html! {
            <Suspense fallback={fallback}>
                <div class="full-image" ref={ref_container.clone()} {onwheel}>
                    <FullPictureImage id={props.id.clone()} {width} {height} zoom={*zoom} {ref_container} {container_width} {container_height}/>
                </div>
            </Suspense>
        });
    }

    warn!("No cached dimensions for image {}", props.id);
    Ok(html! { <div></div> })
}

#[derive(Properties, PartialEq)]
pub struct ImageProps {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub zoom: f64,
    pub ref_container: NodeRef,
    // Needs to be passed as a prop to force re-render when the container size change.
    pub container_width: u32,
    pub container_height: u32,
}

#[allow(non_snake_case)]
#[function_component]
fn FullPictureImage(props: &ImageProps) -> HtmlResult {
    let static_ctx = use_context::<StaticContext>().unwrap();

    let update = use_update();
    let is_first_mount = use_is_first_mount();
    let ref_image = use_node_ref();

    let container = props.ref_container.cast::<HtmlElement>();
    let image = ref_image.cast::<HtmlElement>();

    if is_first_mount {
        update();
        // Force a first empty render to initialize node ref
        return Ok(html! { <div class="image"></div> });
    }

    let mut left = 0;
    let mut top = 0;
    if container.is_some() && image.is_some() {
        let container = container.unwrap();
        let image = image.unwrap();

        let cont_w = container.client_width() as f64;
        let cont_h = container.client_height() as f64;
        let img_w = image.client_width() as f64;
        let img_h = image.client_height() as f64;

        if cont_w > img_w * props.zoom {
            left = ((cont_w - img_w * props.zoom) / 2.0) as i32;
        }
        if cont_h > img_h * props.zoom {
            top = ((cont_h - img_h * props.zoom) / 2.0) as i32;
        }
    }

    Ok(html! {
        <div class="image" ref={ref_image}
            style={format!("background-image: url({}/get-image?id={}&window={}); aspect-ratio: {}/{}; scale: {}; left: {}px; top: {}px;",
            static_ctx.protocol, props.id, static_ctx.window_label, props.width, props.height, props.zoom, left, top)}>
        </div>
    })
}
