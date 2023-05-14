use crate::header::menubar::MenuBar;
use crate::utils::logger::info;
use crate::app::Context;
use crate::utils::utils::{cmd, cmd_async};
use serde::{Serialize, Deserialize};
use tauri_sys::window::current_window;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub class: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GreetArgs<'a> {
    pub name: &'a str,
}

#[function_component]
pub fn Header(props: &Props) -> Html {
    let macos = use_context::<Context>().unwrap().macos;

    let on_minimize = Callback::from(move |_: MouseEvent| {
        spawn_local(async {
             current_window().minimize().await.expect("failed to minimize window");
        });
    });

    let on_maximize = Callback::from(move |_: MouseEvent| {
        spawn_local(async {
            current_window().toggle_maximize().await.expect("failed to minimize window");
        });
    });

    let on_close = Callback::from(move |_: MouseEvent| {
        cmd("menu_close_window");
    });

    let on_greet = Callback::from(move |_: MouseEvent| {
        spawn_local(async {
            let new_msg = cmd_async::<_, String>("greet", &GreetArgs { name: &*"test" }).await;
            info(new_msg.as_str());
        });
    });

    html! {
        <>
            <header data-tauri-drag-region="true" class={classes!(props.class.clone())}>
                {
                    if macos {
                        html! {
                            <>
                                <div class="macos-spacer" data-tauri-drag-region="true"/>
                                <div class="title" data-tauri-drag-region="true">
                                    <p class="title" data-tauri-drag-region="true">{"PictureFiler"}</p>
                                    <p class="path" data-tauri-drag-region="true">{"~/Downloads/Gallery/"}</p>
                                </div>
                            </>
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


                <div class="buttons" data-tauri-drag-region="true">
                    {
                        // Some buttons are only available on Macos, they are duplicates from the menubar
                        if macos {
                            html! {
                                <>
                                    <button aria-labelledby="Star">
                                        <i class="fa-solid fa-sidebar"></i>
                                    </button>
                                    <button aria-labelledby="Star">
                                        <i class="fa-brands fa-twitter"></i>
                                    </button>
                                    <button aria-labelledby="Star">
                                        <i class="fa-brands fa-twitter"></i>
                                    </button>
                                    <button aria-labelledby="Star">
                                        <i class="fa-brands fa-twitter"></i>
                                    </button>
                                    <button aria-labelledby="Star">
                                        <i class="fa-brands fa-twitter"></i>
                                    </button>
                                </>
                            }
                        }else{ html!() }
                    }
                    <button onclick={on_greet} tabindex="0" aria-labelledby="Star">
                        <i class="fa-regular fa-star"></i>
                    </button>
                    <button aria-labelledby="Star">
                        <i class="fa-solid fa-sidebar"></i>
                    </button>
                    <button aria-labelledby="Star">
                        <i class="fa-brands fa-twitter"></i>
                    </button>
                    <button aria-labelledby="Star">
                        <i class="fa-brands fa-twitter"></i>
                    </button>
                    <button aria-labelledby="Star">
                        <i class="fa-brands fa-twitter"></i>
                    </button>
                    <button aria-labelledby="Star">
                        <i class="fa-brands fa-twitter"></i>
                    </button>
                </div>

                {
                     if !macos {
                        html! {
                            <>
                                <div class="fixed-spacer" data-tauri-drag-region="true"/>
                                <div class="title" data-tauri-drag-region="true">
                                    <p class="title" data-tauri-drag-region="true">{"PictureFiler"}</p>
                                    <p class="path" data-tauri-drag-region="true">{"~/Downloads/Gallery/"}</p>
                                </div>
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
