use yew::prelude::*;
use crate::app::{Context};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn Header() -> Html {

    let macos = use_context::<Context>().unwrap().macos;

    html! {
        <>
            <header data-tauri-drag-region="true">
                
                {
                    if macos {
                        html! {
                            <div class="macos-spacer">
                                {{"Macos"}}
                            </div>
                        }
                    } else {
                        html! {
                            <>
                                <div class="windows-icon">
                                    {{"Windows"}}
                                </div>
                                <div class="windows-menu">

                                </div>
                            </>
                        }
                    }
                }
                <div class="buttons">

                </div>
                {
                     if !macos {
                        html! {
                            <div class="windows-buttons">

                            </div>
                        }
                    }else{ html!() }
                }
            </header>
        </>
    }
}
