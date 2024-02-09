use log::warn;
use serde::{Deserialize, Serialize};
use yew::suspense::Suspense;
use yew::{function_component, html, suspense::use_future_with, use_context, HtmlResult, Properties};
use yewdux::Dispatch;

use crate::app::{Context, MainPaneDisplayType};
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
pub fn PictureCarousel(props: &Props) -> HtmlResult {
    let dimensions = use_future_with(props.id.clone(), |id| async move {
        cmd_async::<GetImageArgs, Option<(u32, u32)>>("get_image_dimensions", &GetImageArgs { id: id.to_string() }).await
    })?;

    let carousel_height = 60;

    // Switch to another picture of the carousel on click
    let context_dispatch = Dispatch::<Context>::global();
    let onclick = {
        let id = props.id.clone();
        context_dispatch.reduce_mut_callback(move |data| {
            if let MainPaneDisplayType::PictureAndCarousel(old_id, old_left, mut old_right) = data.main_pane_content.clone() {
                if id == old_id {
                    return;
                }
                let (mut right, mut left) = (vec![], vec![]);
                if let Some(i) = old_left.iter().position(|x| *x == id) {
                    left = old_left[..i].to_vec();
                    right = old_left[i + 1..].to_vec();
                    right.push(old_id);
                    right.append(&mut old_right);
                } else if let Some(i) = old_right.iter().position(|x| *x == id) {
                    left = old_left;
                    left.push(old_id);
                    left.append(&mut old_right[..i].to_vec());
                    right = old_right[i + 1..].to_vec();
                }
                data.main_pane_content = MainPaneDisplayType::PictureAndCarousel(id.clone(), left, right);
            }
        })
    };

    if let Some((width, height)) = *dimensions {
        let fallback = html! {
            <li class="loading" onclick={onclick.clone()}>
                <div class="image" style={format!("width: {}px; height: {}px;", carousel_height*width/height, carousel_height)}/>
            </li>
        };
        return Ok(html! {
            <Suspense fallback={fallback}>
                <li onclick={onclick}>
                    <PictureCarouselImage id={props.id.clone()} width={carousel_height*width/height} height={carousel_height}/>
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
}
#[allow(non_snake_case)]
#[function_component]
fn PictureCarouselImage(props: &ImageProps) -> HtmlResult {
    let static_ctx = use_context::<StaticContext>().unwrap();

    let has_thumb = use_future_with(props.id.clone(), |id| async move {
        cmd_async::<crate::mainpane::full_picture::GetImageArgs, bool>(
            "gen_image_thumbnail",
            &crate::mainpane::full_picture::GetImageArgs { id: id.to_string() },
        )
        .await
    })?;

    if !*has_thumb {
        warn!("No thumb for {}", props.id);
        return Ok(html! {});
    }

    let protocol = if !static_ctx.windows {
        "reqimg://localhost"
    } else {
        "https://reqimg.localhost"
    };

    Ok(html! {
        <div class="image"
            style={format!("background-image: url({}/get-thumbnail?id={}&window={}); width: {}px; height: {}px;",
            protocol, props.id, static_ctx.window_label, props.width, props.height)}>
        </div>
    })
}
