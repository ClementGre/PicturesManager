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
pub fn PictureCarousel(props: &Props) -> HtmlResult {
    let dimensions = use_future_with(props.id.clone(), |id| async move {
        cmd_async::<GetImageArgs, Option<(u32, u32)>>("get_image_dimensions", &GetImageArgs { id: id.to_string() }).await
    })?;

    let carousel_height = 60;

    if let Some((width, height)) = *dimensions {
        let fallback = html! {
            <li class="loading">
                <div class="image" style={format!("width: {}px; height: {}px;", carousel_height*width/height, carousel_height)}/>
            </li>
        };

        // Switch to carousel mode on click
        // let context_dispatch = Dispatch::<Context>::global();
        // let onclick = {
        //     let id = props.id.clone();
        //     context_dispatch.reduce_mut_callback(move |data| {
        //         data.main_pane_content = MainPaneDisplayType::PictureAndCarousel(id.clone(), vec![], vec![]);
        //     })
        // };

        return Ok(html! {
            <Suspense fallback={fallback}>
                <li>
                    <PictureCarouselImage id={props.id.clone()} width={carousel_height*width/height} height={carousel_height}/>
                </li>
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
