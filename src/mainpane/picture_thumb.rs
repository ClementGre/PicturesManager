use serde::{Deserialize, Serialize};
use yew::suspense::Suspense;
use yew::{function_component, html, suspense::use_future_with_deps, use_context, HtmlResult, Properties};

use crate::{
    app::StaticContext,
    utils::{logger::warn, utils::cmd_async},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetImageThumbnailArgs {
    id: String,
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

    if let Some((width, height)) = *dimensions {
        let h = 140;
        let w = h * width / height;

        let fallback = html! {
            <div class="thumb"
                style={format!("aspect-ratio: {} / {};", w, h)}>
            </div>
        };

        return Ok(html! {
            <li style={format!("flex-basis: {}px; flex-grow: {};", w, w)}>
                <Suspense fallback={fallback}>
                    <PictureThumbImage id={props.id.clone()} width={w} height={h}/>
                </Suspense>
            </li>
        });
    }

    warn(format!("No thumb for {}", props.id).as_str());
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
fn PictureThumbImage(props: &ImageProps) -> HtmlResult {
    let static_ctx = use_context::<StaticContext>().unwrap();

    let has_thumb = use_future_with_deps(
        |id| async move { cmd_async::<GetImageThumbnailArgs, bool>("gen_image_thumbnail", &GetImageThumbnailArgs { id: id.to_string() }).await },
        props.id.clone(),
    )?;

    if !*has_thumb {
        warn(format!("No thumb for {}", props.id).as_str());
        return Ok(html! {});
    }

    Ok(html! {
        <div class="thumb"
            style={format!("background-image: url(reqimg://get-thumbnail/?id={}&window={}); aspect-ratio: {} / {};",
            props.id, static_ctx.window_label, props.width, props.height)}>
        </div>
    })
}
