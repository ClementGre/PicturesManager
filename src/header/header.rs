use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

#[function_component]
pub fn Header() -> Html {

    let macos = true;
    

    html! {
        <>
            <header data-tauri-drag-region="true">
                
                {
                    if macos {
                        html! {
                            <div class="macos-spacer">
                            </div>
                        }
                    } else {
                        html! {
                            <>
                                <div class="windows-icon">

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
                     if cfg!(not(target_os = "macos")) {
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
