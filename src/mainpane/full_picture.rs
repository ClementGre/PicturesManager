use log::warn;
use serde::{Deserialize, Serialize};
use yew::suspense::Suspense;
use yew::{function_component, html, suspense::use_future_with, use_context, HtmlResult, Properties};

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

    if let Some((width, height)) = *dimensions {
        let fallback = html! {
            <div class="full-image loading">
                <div class="image" style={format!("aspect-ratio: {} / {};", width, height)}/>
            </div>
        };

        return Ok(html! {
            <Suspense fallback={fallback}>
                <div class="full-image ">
                    <FullPictureImage id={props.id.clone()} {width} {height}/>
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
}
#[allow(non_snake_case)]
#[function_component]
fn FullPictureImage(props: &ImageProps) -> HtmlResult {
    let static_ctx = use_context::<StaticContext>().unwrap();

    let protocol = if !static_ctx.windows {
        "reqimg://localhost"
    } else {
        "https://reqimg.localhost"
    };

    Ok(html! {
        <div class="image-container" /*style={format!("aspect-ratio: {} / {};", props.width, props.height)}*/>
            <div class="image"
                style={format!("background-image: url({}/get-image?id={}&window={}); aspect-ratio: {} / {};",
                protocol, props.id, static_ctx.window_label, props.width, props.height)}>
            </div>
        </div>
    })
}
