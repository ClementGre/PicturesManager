use crate::app::StaticContext;
use crate::header::menubar::MenuBar;
use crate::utils::logger::info;
use crate::utils::utils::{cmd, cmd_async, cmd_arg};
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
    pub name: &'a str
}
#[derive(Serialize, Deserialize)]
pub struct SetSettingsArgs {
    pub settings: Settings
}


#[function_component]
pub fn Header(props: &Props) -> Html {

    let static_ctx = use_context::<StaticContext>().unwrap();
    
    let on_minimize = Callback::from(move |_: MouseEvent| {
        spawn_local(async {
            current_window().minimize().await.expect("failed to minimize window");
        });
    });

    let on_maximize = Callback::from(move |_: MouseEvent| {
        spawn_local(async {
            current_window().toggle_maximize().await.expect("failed to toggle maximize window");
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

    // Settings actions

    let (settings, settings_dispatch) = use_store::<Settings>();
    let switch_language = settings_dispatch.reduce_mut_callback(|settings| {
        if settings.language == Some("fr".to_string()) {
            settings.language = Some("en".to_string())
        } else {
            settings.language = Some("fr".to_string())
        }
    });
    let theme_light = {
        let settings = settings.clone();
        Callback::from(move |_| {
            let mut settings = (*settings).clone();
            settings.theme = Theme::Light;
            cmd_arg("set_settings", &SetSettingsArgs{settings});
        })
    };
    let theme_dark = {
        let settings = settings.clone();
        Callback::from(move |_| {
            let mut settings = (*settings).clone();
            settings.theme = Theme::Dark;
            cmd_arg("set_settings", &SetSettingsArgs{settings});
        })
    };
    let theme_os = {
        let settings = settings.clone();
        Callback::from(move |_| {
            let mut settings = (*settings).clone();
            settings.theme = Theme::System;
            cmd_arg("set_settings", &SetSettingsArgs{settings});
        })
    };

    let toggle_force_win_header = {
        let settings = settings.clone();
        Callback::from(move |_| {
            let mut settings = (*settings).clone();
            settings.force_win_header = !settings.force_win_header;
            cmd_arg("set_settings", &SetSettingsArgs{settings});
        })
    };

    // Context

    let macos_header = {
        let static_ctx = static_ctx.clone();
        let settings = settings.clone();
        use_memo(|settings| {
            static_ctx.macos && !settings.force_win_header
        }, settings)
    };


    html! {
        <>
            <header data-tauri-drag-region="true" class={classes!(props.class.clone())}>
                {
                    if *macos_header {
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
                <div class={ if *macos_header {"spacer"} else { "fixed-spacer" }} data-tauri-drag-region="true"/>


                <div class="buttons" data-tauri-drag-region="true">
                    {
                        // Some buttons are only available on Macos, they are duplicates from the menubar
                        if *macos_header {
                            html! {
                                <>
                                    <button aria-labelledby="Star">
                                        <Icon icon_id={IconId::FontAwesomeSolidStar} />
                                    </button>
                                    <button title="Star">
                                        <Icon icon_id={IconId::FontAwesomeSolidStar} />
                                    </button>
                                    <button title="Star">
                                        <Icon icon_id={IconId::FontAwesomeSolidStar} />
                                    </button>
                                    <button title="Star">
                                        <Icon icon_id={IconId::FontAwesomeSolidStar} />
                                    </button>
                                    <button title="Star">
                                        <Icon icon_id={IconId::FontAwesomeSolidStar} />
                                    </button>
                                </>
                            }
                        }else{ html!() }
                    }
                    <button onclick={theme_light} title="Light Theme">
                        <Icon icon_id={IconId::FontAwesomeSolidSun} />
                    </button>
                    <button onclick={theme_dark} title="Dark Theme">
                        <Icon icon_id={IconId::FontAwesomeSolidMoon} />
                    </button>
                    <button onclick={theme_os} title="Os Theme">
                        <Icon icon_id={IconId::BootstrapHouseGearFill} />
                    </button>
                    <button onclick={switch_language} title="Switch Language">
                        {
                            if settings.language == Some("fr".to_string()) {
                                html! { <Icon icon_id={IconId::FontAwesomeSolidEarthEurope} /> }
                            } else {
                                html! { <Icon icon_id={IconId::FontAwesomeSolidEarthAmericas} /> }
                            }
                        }
                    </button>
                    <button onclick={on_greet} title="Greet">
                        <Icon icon_id={IconId::FontAwesomeSolidMessage} />
                    </button>
                    {
                        if static_ctx.macos {
                            html! {
                                <button onclick={toggle_force_win_header} title="Change Os">
                                    <Icon icon_id={IconId::BootstrapWindows} />
                                </button>
                            }
                        } else {
                            html!()
                        }
                    }
                    <button>
                        <Icon icon_id={IconId::FontAwesomeSolidStar} />
                    </button>

                </div>

                {
                     if !*macos_header {
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
