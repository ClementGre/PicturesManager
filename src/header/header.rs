use crate::header::menubar::MenuBar;
use crate::{app::Context, invoke};
use pm_common::data_structs::Theme;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn Header() -> Html {
    let macos = use_context::<Context>().unwrap().macos;
    let is_light_theme = use_context::<Context>().unwrap().theme != Theme::DARK;
    let theme = use_context::<Context>().unwrap().theme;

    let on_minimize = Callback::from(move |_: MouseEvent| {
        invoke("window_minimize", JsValue::default());
    });

    let on_maximize = Callback::from(move |_: MouseEvent| {
        invoke("window_maximize", JsValue::default());
    });

    let on_close = Callback::from(move |_: MouseEvent| {
        invoke("window_close", JsValue::default());
    });

    html! {
        <>
            <header data-tauri-drag-region="true" class={classes!(if is_light_theme {Some("th-light")} else {None})}>
                {
                    if macos && false {
                        html! {
                            <div class="macos-spacer" data-tauri-drag-region="true"/>
                        }
                    } else {
                        html! {
                            <>
                                <div class="macos-spacer" data-tauri-drag-region="true"/>
                                {format!("theme: {:?}", theme)}
                                <div class="windows-icon" data-tauri-drag-region="true">
                                    <img src="public/yew.png" alt="app icon" data-tauri-drag-region="true" />
                                </div>
                                <MenuBar/>
                            </> 
                        }
                    }
                }
                <div class="buttons" data-tauri-drag-region="true">

                </div>
                <div class="spacer" data-tauri-drag-region="true"/>
                {
                     if !macos || true {
                        html! {
                            <div class="windows-buttons" data-tauri-drag-region="true">
                                <div class="minimize" onclick={on_minimize}>
                                    <div></div>
                                </div>
                                <div class="maximize" onclick={on_maximize}>
                                    <div></div>
                                </div>
                                <div class="close" onclick={on_close}>
                                    <div class="first"></div>
                                    <div class="second"></div>
                                </div>
                            </div>
                        }
                    }else{ html!() }
                }
            </header>
        </>
    }
}
