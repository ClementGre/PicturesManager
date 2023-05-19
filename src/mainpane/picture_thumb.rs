use js_sys::{Array, Uint8Array};
use serde::{Deserialize, Serialize};
use web_sys::{Blob, BlobPropertyBag, Url};
use yew::{function_component, html, suspense::use_future_with_deps, HtmlResult, Properties, use_effect};

use crate::utils::{logger::info, utils::cmd_async};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetImageThumbnailArgs {
    id: String,
}

#[function_component]
pub fn PictureThumb(props: &Props) -> HtmlResult {
    let url = use_future_with_deps(
        |id| async move {
            info("Requesting image thumbnail");
            let data = cmd_async::<GetImageThumbnailArgs, Option<Vec<u8>>>("get_image_thumbnail", &GetImageThumbnailArgs { id: id.to_string() })
                .await
                .unwrap_or_default();

            let u8array = Uint8Array::from(data.as_slice());
            let array = Array::new();
            array.push(&u8array);

            let blob = Blob::new_with_u8_array_sequence_and_options(&array, BlobPropertyBag::new().type_("image/png")).unwrap();
            let url = Url::create_object_url_with_blob(&blob).unwrap();
            url
        },
        props.id.clone(),
    )?;

    {
        let url = url.clone();
        use_effect(move || {
            // Perform the cleanup
            move || {
                let res = Url::revoke_object_url(&url);
                if res.is_err() {
                    info(format!("Failed to revoke object url: {}", &url).as_str());
                }else{
                    info(format!("Revoked object url: {}", &url).as_str());
                }
            }
        });
    }

    Ok(html! {
        <li style={format!("background-image: url({});", *url)}>
        </li>
    })
}
