use gloo::events::EventListener;
use log::info;
use unidecode::unidecode;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    header::menu::{get_menus, MenuItem},
    header::menu_item_component::MenuItemComponent,
    utils::{keystroke::KeyStroke, translator::Translator, utils::cmd},
};

fn register_shortcuts(items: &Vec<MenuItem>, shortcuts: &mut Vec<(KeyStroke, String)>) {
    items.into_iter().for_each(|item| {
        if let Some(accelerator) = item.accelerator.clone() {
            shortcuts.push((KeyStroke::from(accelerator.as_str()), item.id.clone()));
        }
        if let Some(items) = item.items.clone() {
            register_shortcuts(&items, shortcuts);
        }
    });
}

pub fn extract_key_from_text(s: &str) -> char {
    return unidecode(s.to_lowercase().as_str()).chars().next().unwrap_or(' ');
}

#[derive(Clone, Debug, PartialEq)]
pub enum NavigationMessage {
    None,
    Up,
    Down,
    Left,
    Right,
    Fire,
    Close,
    LeftRoot,  // Called when Left is not possible on children item
    RightRoot, // Called when Right is not possible on children item
    CloseRoot, // Called to close root menu properly (meaning closing submenus too)
    Alt(char),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NavigationMessageResult {
    Ignored,
    Consumed,
    ConsumedAndClose, // Requests menu to be closed
}

#[allow(non_snake_case)]
#[function_component]
pub fn MenuBar() -> Html {
    let (t, _) = use_store::<Translator>();
    let menus = get_menus(&t);

    let mut shortcuts = vec![];
    register_shortcuts(&menus, &mut shortcuts);

    let mut alt_shortcuts = vec![];
    menus.iter().for_each(|menu| {
        let name = menu.name.clone().unwrap_or(String::new());
        let split = name.split("_");
        if split.clone().count() >= 2 {
            alt_shortcuts.push((extract_key_from_text(&split.skip(1).next().unwrap()), menu.id.clone()));
        }
    });

    let is_open = use_state_eq(|| false);
    let selected_item = use_state_eq(|| String::new());
    let opened_menu = use_state_eq(|| String::new());
    let bar_ref = use_node_ref();

    let alt_down = use_state_eq(|| false);
    let is_alt_mode = use_state_eq(|| false);
    // True if alt has been used by an action (opening menu or selecting menu). If false, alt release will close menu.
    let is_alt_consumed = use_state_eq(|| false);

    let navigation_message = use_state_eq(|| NavigationMessage::None);
    let navigation_message_received = {
        let is_alt_mode = is_alt_mode.clone();
        let is_alt_consumed = is_alt_consumed.clone();
        let alt_down = alt_down.clone();
        let selected_item = selected_item.clone();
        let bar_ref = bar_ref.clone();
        let navigation_message = navigation_message.clone();
        Callback::from(move |result: NavigationMessageResult| {
            let mut new_message = NavigationMessage::None;
            if let NavigationMessage::Alt(_) = *navigation_message {
                if result != NavigationMessageResult::Ignored {
                    is_alt_mode.set(true);
                    if *alt_down {
                        is_alt_consumed.set(true);
                    }

                    if result == NavigationMessageResult::ConsumedAndClose {
                        bar_ref.cast::<HtmlElement>().unwrap().blur().unwrap();
                        is_alt_mode.set(false);
                        selected_item.set(String::new());
                        new_message = NavigationMessage::CloseRoot; // Will automatically close menu bar
                    }
                }
            }
            navigation_message.set(new_message);
        })
    };
    let send_navigation_message = {
        let navigation_message = navigation_message.clone();
        Callback::from(move |new_nm: NavigationMessage| {
            navigation_message.set(new_nm);
        })
    };

    // Global keydown event
    {
        let shortcuts = shortcuts.clone();
        let alt_down = alt_down.clone();
        let is_alt_mode = is_alt_mode.clone();
        let is_alt_consumed = is_alt_consumed.clone();
        let bar_ref = bar_ref.clone();
        use_effect(move || {
            let document = gloo::utils::document();
            let listener = EventListener::new(&document, "keydown", move |e| {
                let e = e.dyn_ref::<KeyboardEvent>().unwrap();

                // Keyboard shortcuts
                shortcuts.clone().iter().for_each(|(ks, id)| {
                    if ks.matches(&e) {
                        cmd(format!("menu_{}", id).as_str());
                        return;
                    }
                });

                // Alt pressed
                if e.key() == "Alt" {
                    // Alt pressed alone
                    if *alt_down {
                        return;
                    }
                    alt_down.set(true);

                    if !*is_alt_mode {
                        is_alt_consumed.set(true);
                        is_alt_mode.set(true);
                        bar_ref.cast::<HtmlElement>().unwrap().focus().unwrap();
                    }
                }
            });
            // Called when the component is unmounted.  The closure has to hold on to `listener`, because if it gets
            // dropped, `gloo` detaches it from the DOM. So it's important to do _something_, even if it's just dropping it.
            || drop(listener)
        });
    }
    // Global keyup event
    {
        let is_alt_mode = is_alt_mode.clone();
        let selected_item = selected_item.clone();
        let alt_down = alt_down.clone();
        let is_alt_consumed = is_alt_consumed.clone();
        let bar_ref = bar_ref.clone();
        let navigation_message = navigation_message.clone();
        use_effect(move || {
            let document = gloo::utils::document();
            let listener = EventListener::new(&document, "keyup", move |e| {
                let e = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                // Alt key
                if e.key() == "Alt" {
                    alt_down.set(false);

                    if *is_alt_mode && !*is_alt_consumed {
                        bar_ref.cast::<HtmlElement>().unwrap().blur().unwrap();
                        is_alt_mode.set(false);
                        navigation_message.set(NavigationMessage::CloseRoot); // Will automatically close menu bar
                        selected_item.set(String::new());
                    }
                    is_alt_consumed.set(false);
                }
            });
            || drop(listener)
        });
    }
    // Global mousedown event
    {
        let selected_item = selected_item.clone();
        let is_alt_mode = is_alt_mode.clone();
        let navigation_message = navigation_message.clone();
        use_effect(move || {
            let document = gloo::utils::document();
            let listener = EventListener::new(&document, "mousedown", move |e| {
                let e = e.dyn_ref::<web_sys::MouseEvent>().unwrap();

                let target = e.target().and_then(|div| div.dyn_into::<HtmlElement>().ok());
                if let Some(div) = target {
                    info!("target: {}", div.class_name());
                    if !div.class_name().split_whitespace().any(|c| "menu-item" == c || "menu" == c) {
                        // Close menu only if the target is not a menu-item (menu will otherwise be closed on click RELEASE)
                        is_alt_mode.set(false);
                        navigation_message.set(NavigationMessage::CloseRoot); // Will automatically close menu bar
                        selected_item.set(String::new());
                    }
                }
            });
            || drop(listener)
        });
    }

    let opened_at_click_time = use_state_eq(|| None);

    let onmousedown = {
        let is_open = is_open.clone();
        let opened_at_click_time = opened_at_click_time.clone();
        let bar_ref = bar_ref.clone();
        Callback::from(move |_: MouseEvent| {
            opened_at_click_time.set(Some(*is_open));
            if !*is_open {
                is_open.set(true);
                bar_ref.cast::<HtmlElement>().unwrap().focus().unwrap();
            }
        })
    };
    let onmouseup = {
        let opened_at_click_time = opened_at_click_time.clone();
        let is_alt_mode = is_alt_mode.clone();
        let navigation_message = navigation_message.clone();
        let selected_item = selected_item.clone();
        let bar_ref = bar_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if *opened_at_click_time == Some(true) {
                is_alt_mode.set(false);
                navigation_message.set(NavigationMessage::CloseRoot); // Will automatically close menu bar
                selected_item.set(String::new());
                bar_ref.cast::<HtmlElement>().unwrap().blur().unwrap();
            }
            opened_at_click_time.set(None);
        })
    };
    let onkeydown = {
        let alt_shortcuts = alt_shortcuts.clone();
        let is_open = is_open.clone();
        let selected_item = selected_item.clone();
        let opened_menu = opened_menu.clone();
        let alt_down = alt_down.clone();
        let is_alt_mode = is_alt_mode.clone();
        let is_alt_consumed = is_alt_consumed.clone();
        let navigation_message = navigation_message.clone();
        let bar_ref = bar_ref.clone();
        Callback::from(move |e: KeyboardEvent| {
            let mut found_target = false;

            match e.key().as_str() {
                "Escape" => {
                    if !*is_open {
                        bar_ref.cast::<HtmlElement>().unwrap().blur().unwrap();
                    } else {
                        navigation_message.set(NavigationMessage::Close);
                    }
                }
                "Enter" | " " => {
                    if !*is_open {
                        is_open.set(true)
                    }
                    navigation_message.set(NavigationMessage::Fire);
                }
                "ArrowUp" => {
                    navigation_message.set(NavigationMessage::Up);
                }
                "ArrowDown" => {
                    if !*is_open {
                        is_open.set(true)
                    }
                    navigation_message.set(NavigationMessage::Down);
                }
                "ArrowLeft" => {
                    if !*is_open {
                        is_open.set(true)
                    }
                    navigation_message.set(NavigationMessage::Left);
                }
                "ArrowRight" => {
                    if !*is_open {
                        is_open.set(true)
                    }
                    navigation_message.set(NavigationMessage::Right);
                }
                _ => {
                    if e.key().as_str() != "Alt" {
                        let key = extract_key_from_text(e.key().as_str());

                        // Alt shortcuts for root menus navigation:
                        if !*is_open || opened_menu.is_empty() {
                            alt_shortcuts.clone().iter().for_each(|(ks, id)| {
                                if *ks == key {
                                    is_open.set(true);
                                    selected_item.set(id.clone());
                                    opened_menu.set(id.clone());
                                    found_target = true;
                                }
                            });
                        }

                        if !found_target {
                            navigation_message.set(NavigationMessage::Alt(key));
                            // The back event will always be called and will change is_alt_consumed from navigation_message_callback
                        } else {
                            is_alt_mode.set(true);
                            if *alt_down {
                                is_alt_consumed.set(true);
                            }
                            return;
                        }
                    }
                }
            }
        })
    };

    let onfocus = {
        let menus = menus.clone();
        let opened_at_click_time = opened_at_click_time.clone();
        let selected_item = selected_item.clone();
        let opened_menu = opened_menu.clone();
        let navigation_message = navigation_message.clone();
        Callback::from(move |_: FocusEvent| {
            if *opened_at_click_time == None {
                // Oppened by keyboard tab navigation
                if selected_item.is_empty() {
                    selected_item.set(menus.clone()[0].id.clone());
                    if !opened_menu.is_empty() {
                        navigation_message.set(NavigationMessage::CloseRoot); // Will automatically close menu bar
                    }
                } else {
                    opened_menu.set((*selected_item).clone());
                }
            }
        })
    };
    let onfocusout = {
        let selected_item = selected_item.clone();
        let is_alt_mode = is_alt_mode.clone();
        let navigation_message = navigation_message.clone();
        Callback::from(move |_: _| {
            if *opened_at_click_time == None {
                navigation_message.set(NavigationMessage::CloseRoot); // Will automatically close menu bar
                selected_item.set(String::new());
            }
            is_alt_mode.set(false);
        })
    };

    let update_children_selected_item = {
        let selected_item = selected_item.clone();
        Callback::from(move |id: String| {
            selected_item.set(id);
        })
    };
    let update_children_opened_menu = {
        let opened_menu = opened_menu.clone();
        let is_open = is_open.clone();
        Callback::from(move |id: String| {
            if id.is_empty() {
                // Automatically close the menu bar when the last menu is closed
                is_open.set(false);
            }
            opened_menu.set(id);
        })
    };

    let brothers = menus.iter().map(|menu| menu.id.clone()).collect::<Vec<String>>();
    html! {
        <div
            id="app-menu-bar"
            ref={bar_ref.clone()}
            tabindex="0"
            class={classes!("windows-menu", if *is_open.clone() {Some("opened")} else {None}, if *is_alt_mode {Some("alt-mode")} else {None})}
            {onmousedown} {onmouseup} {onkeydown} {onfocus} {onfocusout}>
            {
                menus.into_iter().map(|item| {
                    html!{
                        <MenuItemComponent
                            item={item}
                            is_root={true}
                            is_open={*is_open.clone()}
                            selected_item={(*selected_item).clone()}
                            opened_menu={(*opened_menu).clone()}
                            brothers={brothers.clone()}
                            update_selected_item={update_children_selected_item.clone()}
                            update_opened_menu={update_children_opened_menu.clone()}
                            navigation_message={(*navigation_message).clone()}
                            navigation_message_received={navigation_message_received.clone()}
                            send_navigation_message={send_navigation_message.clone()}
                        />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct MenuTextProps {
    pub text: String,
}

#[allow(non_snake_case)]
#[function_component]
pub fn MenuTextComponent(props: &MenuTextProps) -> Html {
    let text = props.text.clone();

    let mut left_part = String::new();
    let mut shortcut = String::new();
    let mut right_part = String::new();

    let mut is_shortcut = false;

    for c in text.chars() {
        if c == '_' {
            is_shortcut = true;
            left_part = right_part;
            right_part = String::new();
        } else {
            if is_shortcut {
                shortcut = c.to_string();
                is_shortcut = false;
            } else {
                right_part.push(c);
            }
        }
    }

    html! {
        <p>
            {left_part}
            <span>{shortcut}</span>
            {right_part}
        </p>
    }
}
