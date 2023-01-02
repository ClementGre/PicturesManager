use crate::{app::{Context}, invoke};
use wasm_bindgen::JsValue;
use yew::{prelude::*};
use crate::header::menubar::MenuBar;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn Header() -> Html {
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
                <header data-tauri-drag-region="true">
                    {
                        if macos && false {
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
                    <div class="buttons" data-tauri-drag-region="true">
                        
                    </div>
                    <div class="spacer" data-tauri-drag-region="true"/>
                    {
                         if !macos {
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
