use crate::app::Context;
use crate::header::menubar::MenuBar;
use crate::utils::logger::info;
use crate::utils::utils::{cmd, cmd_async};
use pm_common::app_data::{Settings, Theme};
use serde::{Deserialize, Serialize};
use tauri_sys::window::current_window;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yewdux::prelude::use_store;

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

    let (settings, settings_dispatch) = use_store::<Settings>();
    let switch_language = settings_dispatch.reduce_mut_callback(|settings| {
        if settings.language == Some("fr".to_string()) {
            settings.language = Some("en".to_string())
        } else {
            settings.language = Some("fr".to_string())
        }
    });
    let theme_light = settings_dispatch.reduce_mut_callback(|settings| settings.theme = Theme::Light);
    let theme_dark = settings_dispatch.reduce_mut_callback(|settings| settings.theme = Theme::Dark);

    let (_, context_dispatch) = use_store::<Context>();
    let change_os = context_dispatch.reduce_mut_callback(|context| {
        context.macos = !context.macos;
        context.windows = !context.windows;
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
                                        <Icon icon_id={IconId::FontAwesomeSolidStar} />
                                    </button>
                                    <button aria-labelledby="Star">
                                        <Icon icon_id={IconId::FontAwesomeSolidStar} />
                                    </button>
                                    <button aria-labelledby="Star">
                                        <Icon icon_id={IconId::FontAwesomeSolidStar} />
                                    </button>
                                    <button aria-labelledby="Star">
                                        <Icon icon_id={IconId::FontAwesomeSolidStar} />
                                    </button>
                                    <button aria-labelledby="Star">
                                        <Icon icon_id={IconId::FontAwesomeSolidStar} />
                                    </button>
                                </>
                            }
                        }else{ html!() }
                    }
                    <button onclick={theme_light} aria-labelledby="Light Theme">
                        <Icon icon_id={IconId::FontAwesomeSolidSun} />
                    </button>
                    <button onclick={theme_dark} aria-labelledby="Dark Theme">
                        <Icon icon_id={IconId::FontAwesomeSolidMoon} />
                    </button>
                    <button onclick={switch_language} aria-labelledby="Switch Language">
                        {
                            if settings.language == Some("fr".to_string()) {
                                html! { <Icon icon_id={IconId::FontAwesomeSolidEarthEurope} /> }
                            } else {
                                html! { <Icon icon_id={IconId::FontAwesomeSolidEarthAmericas} /> }
                            }
                        }
                    </button>
                    <button onclick={on_greet} aria-labelledby="Greet">
                        <Icon icon_id={IconId::FontAwesomeSolidMessage} />
                    </button>
                    <button onclick={change_os} aria-labelledby="Change Os">
                        <Icon icon_id={IconId::BootstrapWindows} />
                    </button>
                    <button>
                        <Icon icon_id={IconId::FontAwesomeSolidStar} />
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
