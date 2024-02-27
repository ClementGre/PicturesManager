use log::warn;
use web_sys::MouseEvent;
use yew::suspense::Suspense;
use yew::use_node_ref;
use yew::{function_component, html, suspense::use_future_with, use_context, Callback, HtmlResult, Properties};
use yew_hooks::{use_is_first_mount, use_size, use_update};
use yewdux::prelude::use_selector;
use yewdux::Dispatch;

use crate::app::Context;
use crate::mainpane::full_picture::GetImageArgs;
use crate::utils::utils::get_non_null_ref;
use crate::{app::StaticContext, utils::utils::cmd_async};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub id: String,
    pub to_carousel_cb: Callback<usize>,
}

#[allow(non_snake_case)]
#[function_component]
pub fn PictureThumb(props: &Props) -> HtmlResult {
    let dimensions = use_future_with(props.id.clone(), |id| async move {
        cmd_async::<GetImageArgs, Option<(u32, u32)>>("get_image_dimensions", &GetImageArgs { id: id.to_string() }).await
    })?;
    let selected_index = use_selector(|ctx: &Context| ctx.main_pane_selected_index);
    let selected_indices = use_selector(|ctx: &Context| ctx.main_pane_selected_indices.clone());
    let is_primary_selected = selected_index.map_or(false, |index| index == props.index);
    let is_selected = is_primary_selected || selected_indices.contains(&props.index);

    let context_dispatch = Dispatch::<Context>::global();
    let macos = use_context::<StaticContext>().unwrap().macos;

    let main_pane_dimensions = use_selector(|ctx: &Context| ctx.main_pane_dimensions.clone());
    let ref_load = use_node_ref();
    let ref_pic = use_node_ref();
    let update = use_update();
    let is_first_mount = use_is_first_mount();

    // Force component to re-render when the loading element size changes (gets defined).
    let _ = use_size(ref_load.clone());

    if let Some((width, height)) = *dimensions {
        let h = 140;
        let w = h * width / height;

        let onclick = {
            let index = props.index.clone();
            context_dispatch.reduce_mut_callback_with(move |ctx, e: MouseEvent| {
                if e.button() == 0 {
                    ctx.select_index(index, e.shift_key(), (macos && e.meta_key()) || (!macos && e.ctrl_key()));
                } else if e.button() == 2 {
                    // TODO: Context menu
                }
            })
        };
        let ondblclick = {
            let to_carousel_cb = props.to_carousel_cb.clone();
            let index = props.index.clone();
            Callback::from(move |_: MouseEvent| {
                to_carousel_cb.emit(index);
            })
        };

        let fallback = html! {
            <li class="loading" style={format!("flex-basis: {}px; flex-grow: {};", w, w)} ref={ref_load.clone()} onclick={onclick.clone()}>
                <div class="thumb" style={format!("aspect-ratio: {} / {};", w, h)} />
            </li>
        };

        if is_first_mount {
            update();
            // Force a first empty render to initialize node ref
            return Ok(fallback);
        }

        // Not displaying image if not in the visible area
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
                <li style={format!("flex-basis: {}px; flex-grow: {};", w, w)}
                    ref={ref_pic.clone()} onclick={onclick.clone()} ondblclick={ondblclick.clone()}>
                    <PictureThumbImage id={props.id.clone()} width={w} height={h}/>
                    {
                        if is_selected {
                            html! {
                                <div class="selected-overlay"></div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </li>
            </Suspense>
        });
    }

    warn!("No cached dimensions for image {}", props.id);
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

    let has_thumb = use_future_with(props.id.clone(), |id| async move {
        cmd_async::<GetImageArgs, bool>("gen_image_thumbnail", &GetImageArgs { id: id.to_string() }).await
    })?;

    if !*has_thumb {
        warn!("No thumb for {}", props.id);
        return Ok(html! {});
    }

    Ok(html! {
        <div class="thumb"
            style={format!("background-image: url({}/get-thumbnail?id={}&window={}); aspect-ratio: {} / {};",
            static_ctx.protocol, props.id, static_ctx.window_label, props.width, props.height)}>
        </div>
    })
}
