use log::{warn, info};
use serde::{Deserialize, Serialize};
use web_sys::HtmlElement;
use yew::suspense::Suspense;
use yew::{function_component, html, suspense::use_future_with_deps, use_context, HtmlResult, Properties};
use yew::{use_node_ref, NodeRef, use_state, use_memo};
use yew_hooks::{use_is_first_mount, use_update, use_size};
use yewdux::prelude::use_selector;
use crate::app::Context;
use crate::{app::StaticContext, utils::utils::cmd_async};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetImageThumbnailArgs {
    id: String,
}

fn get_non_null_ref(ref_1: NodeRef, ref_2: NodeRef) -> Option<HtmlElement> {
    if let Some(element) = ref_1.cast::<HtmlElement>() {
        return Some(element);
    } else if let Some(element) = ref_2.cast::<HtmlElement>() {
        return Some(element);
    }
    return None;
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}
#[allow(non_snake_case)]
#[function_component]
pub fn PictureThumb(props: &Props) -> HtmlResult {
    let dimensions = use_future_with_deps(
        |id| async move {
            cmd_async::<GetImageThumbnailArgs, Option<(u32, u32)>>("get_image_dimensions", &GetImageThumbnailArgs { id: id.to_string() }).await
        },
        props.id.clone(),
    )?;

    let main_pane_dimensions = use_selector(|ctx: &Context| ctx.main_pane_dimesions.clone());
    let ref_load = use_node_ref();
    let ref_pic = use_node_ref();
    let update = use_update();
    let is_first_mount = use_is_first_mount();

    // Force component to re-render when the loading element size changes (gets defined).
    let _ = use_size(ref_load.clone());

    if let Some((width, height)) = *dimensions {
        let h = 140;
        let w = h * width / height;

        let fallback = html! {
            <li class="loading" style={format!("flex-basis: {}px; flex-grow: {};", w, w)} ref={ref_load.clone()}>
                <div class="thumb" style={format!("aspect-ratio: {} / {};", w, h)} />
            </li>
        };
    
        if is_first_mount {
            update();
            // Force a first empty render to initialize node ref
            return Ok(fallback);
        }

        let element = get_non_null_ref(ref_load.clone(), ref_pic.clone());
        let mut visible = false;
        if let Some(el) = element {
            let top = el.offset_top();
            let height = el.offset_height();
            if main_pane_dimensions.scroll_bottom != 0 && top != 0 && height != 0 {
                // Add 300 px margin to add scroll smoothness
                visible = top + height >= main_pane_dimensions.scroll_top - 300 && top <= main_pane_dimensions.scroll_bottom + 300;
            }
        }

        if !visible { 
            return Ok(fallback);
        }
        return Ok(html! {
            <Suspense fallback={fallback}>
                <li style={format!("flex-basis: {}px; flex-grow: {};", w, w)} ref={ref_pic.clone()}>
                    <PictureThumbImage id={props.id.clone()} width={w} height={h}/>
                </li>
            </Suspense>
        });
    }

    warn!("No thumb for {}", props.id);
    return Ok(html! { <li ref={ref_pic.clone()}></li> });
}

#[derive(Properties, PartialEq)]
pub struct ImageProps {
    pub id: String,
    pub width: u32,
    pub height: u32,
}
#[allow(non_snake_case)]
#[function_component]
fn PictureThumbImage(props: &ImageProps) -> HtmlResult {
    let static_ctx = use_context::<StaticContext>().unwrap();

    let has_thumb = use_future_with_deps(
        |id| async move { cmd_async::<GetImageThumbnailArgs, bool>("gen_image_thumbnail", &GetImageThumbnailArgs { id: id.to_string() }).await },
        props.id.clone(),
    )?;

    if !*has_thumb {
        warn!("No thumb for {}", props.id);
        return Ok(html! {});
    }

    Ok(html! {
        <div class="thumb"
            style={format!("background-image: url(reqimg://get-thumbnail/?id={}&window={}); aspect-ratio: {} / {};",
            props.id, static_ctx.window_label, props.width, props.height)}>
        </div>
    })
}
