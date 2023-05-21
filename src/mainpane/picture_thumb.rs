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
#[function_component]
pub fn PictureThumb(props: &Props) -> HtmlResult {
    let dimensions = use_future_with_deps(
        |id| async move {
            cmd_async::<GetImageThumbnailArgs, Option<(i32, i32)>>("get_image_dimensions", &GetImageThumbnailArgs { id: id.to_string() }).await
        },
        props.id.clone(),
    )?;

    if let Some((width, height)) = *dimensions {
        let h = 140;
        let w = h * width / height;

        return Ok(html! {
            <li style={format!("width: {}px; height: {}px;", w, h)}>
                <div class="thumb-container">
                    <Suspense fallback={html!{}}>
                        <PictureThumbImage id={props.id.clone()}/>
                    </Suspense>
                </div>
            </li>
        });
    }

    warn(format!("No thumb for {}", props.id).as_str());
    return Ok(html! { <li></li> });
}

#[function_component]
fn PictureThumbImage(props: &Props) -> HtmlResult {
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
        <div class="thumb" style={format!("background-image: url(reqimg://get-thumbnail/?id={}&window={})", props.id, static_ctx.window_label)}>
        </div>
    })
}
