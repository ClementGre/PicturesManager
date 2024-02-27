use log::warn;
use yew::suspense::Suspense;
use yew::{function_component, html, suspense::use_future_with, use_context, HtmlResult, Properties};
use yew::{use_node_ref, Html};

use crate::mainpane::full_picture::GetImageArgs;
use crate::{app::StaticContext, utils::utils::cmd_async};

#[derive(Properties, PartialEq)]
pub struct PicturePreviewProps {
    pub id: String,
    pub thumbnail: bool,
}
#[allow(non_snake_case)]
#[function_component]
pub fn PicturePreview(props: &PicturePreviewProps) -> HtmlResult {
    let dimensions = use_future_with(props.id.clone(), |id| async move {
        cmd_async::<GetImageArgs, Option<(u32, u32)>>("get_image_dimensions", &GetImageArgs { id: id.to_string() }).await
    })?;

    let ref_pic = use_node_ref();

    if let Some((width, height)) = *dimensions {
        let fallback = html! {
            <div class="preview loading">
                <div class="thumb" style={format!("aspect-ratio: {} / {};", width, height)} />
            </div>
        };

        return Ok(html! {
            <Suspense fallback={fallback}>
                <div class="preview">
                    {
                        if props.thumbnail {
                            html! { <PictureThumbPreviewImage id={props.id.clone()} width={width} height={height}/> }
                        }else {
                            html! { <PicturePreviewImage id={props.id.clone()} width={width} height={height}/> }
                        }
                    }
                </div>
            </Suspense>
        });
    }

    warn!("No cached dimensions for image {}", props.id);
    return Ok(html! { <li ref={ref_pic.clone()}></li> });
}

#[derive(Properties, PartialEq)]
pub struct PicturesPreviewProps {
    pub ids: Vec<String>,
}
#[allow(non_snake_case)]
#[function_component]
pub fn PicturesPreview(props: &PicturesPreviewProps) -> Html {
    html! {
        <div class="multiple-previews">
            {
                props.ids.iter().map(|id| html! {
                    <PicturePreview id={id.clone()} thumbnail={true} />
                }).collect::<Html>()
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PictureThumbPreviewImageProps {
    pub id: String,
    pub width: u32,
    pub height: u32,
}
#[allow(non_snake_case)]
#[function_component]
fn PictureThumbPreviewImage(props: &PicturePreviewImageProps) -> HtmlResult {
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
            style={format!("background-image: url({}/get-thumbnail?id={}&window={}); /*aspect-ratio: {} / {};*/",
            static_ctx.protocol, props.id, static_ctx.window_label, props.width, props.height)}>
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct PicturePreviewImageProps {
    pub id: String,
    pub width: u32,
    pub height: u32,
}
#[allow(non_snake_case)]
#[function_component]
fn PicturePreviewImage(props: &PicturePreviewImageProps) -> Html {
    let static_ctx = use_context::<StaticContext>().unwrap();
    html! {
        <div class="thumb"
            style={format!("background-image: url({}/get-image?id={}&window={}); aspect-ratio: {} / {};",
            static_ctx.protocol, props.id, static_ctx.window_label, props.width, props.height)}>
        </div>
    }
}
