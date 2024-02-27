use yew::{function_component, html, Children, Html, Properties, Suspense};
use yewdux::use_selector;

use crate::app::Context;
use crate::rightbar::picture_preview::{PicturePreview, PicturesPreview};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[allow(non_snake_case)]
#[function_component]
pub fn RightBar() -> Html {
    let ids = use_selector(|ctx: &Context| ctx.get_selected_picture_ids());

    let fallback = html! {
        <section class="sidebar rightbar">
        </section>
    };

    if ids.len() == 0 {
        html! {
            <section class="sidebar rightbar" style="width: 0; border: none;">
            </section>
        }
    } else if ids.len() == 1 {
        html! {
            <Suspense fallback={fallback}>
                <section class="sidebar rightbar">
                    <PicturePreview id={ids[0].clone()} thumbnail={false}/>
                    <div class="content">

                    </div>
                </section>
            </Suspense>
        }
    } else {
        let mut preview_ids = vec![
            ids[0].clone(),
            ids[ids.len() * 1 / 3].clone(),
            ids[ids.len() * 2 / 3].clone(),
            ids[ids.len() - 1].clone(),
        ];
        preview_ids.dedup();
        html! {
            <Suspense fallback={fallback}>
                <section class="sidebar rightbar">
                    <PicturesPreview ids={preview_ids}/>
                    <div class="content">

                    </div>
                </section>
            </Suspense>
        }
    }
}
