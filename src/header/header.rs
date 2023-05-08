use crate::header::menubar::MenuBar;
use crate::{app::Context, invoke};
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub class: Option<String>,
}

#[function_component]
pub fn Header(props: &Props) -> Html {
    let macos = use_context::<Context>().unwrap().macos;

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
            <header data-tauri-drag-region="true" class={classes!(props.class.clone())}>
                {
                    if macos {
                        html! {
                            <div class="macos-spacer" data-tauri-drag-region="true"/>
                        }
                    } else {
                        html! {
                            <>
                                <div class="macos-spacer" data-tauri-drag-region="true"/>
                                <div class="windows-icon" data-tauri-drag-region="true">
                                    <img src="public/yew.png" alt="app icon" data-tauri-drag-region="true" />
                                </div>
                                <MenuBar/>
                            </>
                        }
                    }
                }
                // Macos buttons are right aligned, but windows buttons are left aligned
                <div class={ if macos {"spacer"} else { "fixed-spacer" }} data-tauri-drag-region="true"/>
                
                // Some buttons are only available on Macos because on Windows, the menu bar is more accessible
                <div class="buttons" data-tauri-drag-region="true">
                    <div class="button" role="button" aria-labelledby="Star">
                        <i class="fa-regular fa-star"></i>
                    </div>
                </div>

                {
                     if !macos {
                        html! {
                            <>
                                <div class="spacer" data-tauri-drag-region="true"/>
                                <div class="windows-buttons" data-tauri-drag-region="true">
                                    <div class="minimize" onclick={on_minimize} role="button" aria-labelledby="Minimize window">
                                        <div></div>
                                    </div>
                                    <div class="maximize" onclick={on_maximize} role="button" aria-labelledby="Maximize window">
                                        <div></div>
                                    </div>
                                    <div class="close" onclick={on_close} role="button" aria-labelledby="Close window">
                                        <div class="first"></div>
                                        <div class="second"></div>
                                    </div>
                                </div>
                            </>
                        }
                    }else{
                        html! {
                            <div class="fixed-spacer" data-tauri-drag-region="true"/>
                        }
                    }
                }
            </header>
        </>
    }
}
