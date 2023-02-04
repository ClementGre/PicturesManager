use crate::{
    header::menu::{get_menus, MenuItem},
    header::menu_item_component::MenuItemComponent,
    invoke,
    utils::{keystroke::KeyStroke, logger::info},
};
use gloo::events::EventListener;
use unidecode::unidecode;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlElement};
use yew::prelude::*;

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

pub fn extract_key_from_text(s: &str) -> String {
    return unidecode(s.to_lowercase().as_str())[..1].to_string();
}

#[derive(Clone, Debug, PartialEq)]
pub enum NavigationMessage{
    None,
    Next,
    Previous,
    Fire,
    Close,
    Alt(char),
}

#[function_component]
pub fn MenuBar() -> Html {
    let menus = get_menus();

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
    let is_alt_mode_opening = use_state_eq(|| false);

    let navigation_message = use_state_eq(|| NavigationMessage::None);
    let navigation_message_received = {
        let is_alt_mode = is_alt_mode.clone();
        let is_alt_mode_opening = is_alt_mode_opening.clone();
        let navigation_message = navigation_message.clone();
        Callback::from(move |ignored: bool| {
            if let NavigationMessage::Alt(_) = *navigation_message {
                if ignored {
                    is_alt_mode_opening.set(false);
                }else {
                    is_alt_mode.set(true);
                    is_alt_mode_opening.set(true);
                }
            }
            navigation_message.set(NavigationMessage::None);
        })
    };

    // Global keydown event
    {
        let is_open = is_open.clone();
        let selected_item = selected_item.clone();
        let opened_menu = opened_menu.clone();
        let shortcuts = shortcuts.clone();
        let alt_shortcuts = alt_shortcuts.clone();
        let alt_down = alt_down.clone();
        let is_alt_mode = is_alt_mode.clone();
        let is_alt_mode_opening = is_alt_mode_opening.clone();
        let bar_ref = bar_ref.clone();
        use_effect(move || {
            let document = gloo::utils::document();
            let listener = EventListener::new(&document, "keydown", move |e| {
                let e = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap();

                shortcuts.clone().iter().for_each(|(ks, id)| {
                    if ks.matches(&e) {
                        invoke(format!("menu_{}", id).as_str(), JsValue::default());
                    }
                });

                if e.key_code() == 18 {
                    // Alt pressed alone
                    if *alt_down {
                        return;
                    }
                    alt_down.set(true);

                    if !*is_alt_mode {
                        is_alt_mode_opening.set(true);
                        bar_ref.cast::<HtmlElement>().unwrap().focus().unwrap();
                    } else {
                        // Closing the menu
                        bar_ref.cast::<HtmlElement>().unwrap().blur().unwrap();
                    }
                    is_alt_mode.set(!*is_alt_mode);

                }else{
                    // Alt + Key
                    let mut found_target = false;
                    alt_shortcuts.clone().iter().for_each(|(ks, id)| {
                        if *ks == extract_key_from_text(e.key().as_str()) {
                            is_open.set(true);
                            selected_item.set(id.clone());
                            opened_menu.set(id.clone());
                            found_target = true;
                        }
                    });
                    if !found_target {
                        is_alt_mode_opening.set(false);
                    }else{
                        is_alt_mode.set(true);
                        is_alt_mode_opening.set(true);
                    }
                }
            });
            // Called when the component is unmounted.  The closure has to hold on to `listener`, because if it gets
            // dropped, `gloo` de taches it from the DOM. So it's important to do _something_, even if it's just dropping it.
            || drop(listener)
        });
    }
    // Global keyup event
    {
        let is_open = is_open.clone();
        let is_alt_mode = is_alt_mode.clone();
        let selected_item = selected_item.clone();
        let opened_menu = opened_menu.clone();
        let alt_down = alt_down.clone();
        let is_alt_mode_opening = is_alt_mode_opening.clone();
        use_effect(move || {
            let document = gloo::utils::document();
            let listener = EventListener::new(&document, "keyup", move |e| {
                let e = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                // Alt key
                if e.key_code() == 18 {
                    alt_down.set(false);

                    if *is_alt_mode && !*is_alt_mode_opening {
                        is_alt_mode.set(false);
                        is_open.set(false);
                        selected_item.set(String::new());
                        opened_menu.set(String::new());
                    }
                    is_alt_mode_opening.set(false);
                }
            });
            || drop(listener)
        });
    }
    // Global mousedown event
    {
        let is_open = is_open.clone();
        let selected_item = selected_item.clone();
        let opened_menu = opened_menu.clone();
        let is_alt_mode = is_alt_mode.clone();
        use_effect(move || {
            let document = gloo::utils::document();
            let listener = EventListener::new(&document, "mousedown", move |e| {
                let e = e.dyn_ref::<web_sys::MouseEvent>().unwrap();

                let target = e.target().and_then(|div| div.dyn_into::<HtmlElement>().ok());
                if let Some(div) = target {
                    info(format!("target: {}", div.class_name()).as_str());
                    if !div.class_name().split_whitespace().any(|c| "menu-item" == c || "menu" == c) {
                        is_open.set(false); // Close menu only if the target is not a menu-item
                        is_alt_mode.set(false);
                        selected_item.set(String::new());
                        opened_menu.set(String::new());
                    }
                }
            });
            || drop(listener)
        });
    }

    /*
    let on_key_press = {
        let id = menu.id.clone();
        let is_menu = menu.items.is_some();
        let is_root = is_root.clone();
        let is_open = ctxt.is_open.clone();
        let opened_menus = ctxt.opened_menus.clone();
        let selected_item = ctxt.selected_item.clone();
        let item_ref = item_ref.clone();

        Callback::from(move |e: KeyboardEvent| {
            // Enter key
            if e.key_code() == 13 {
                e.prevent_default();
                e.stop_propagation();
                if is_menu {
                    if *selected == id.clone() {
                        selected.set(String::new());
                        if is_root {
                            is_open.clone().set(false);
                        }
                    } else {
                        selected.set(id.clone());
                        if is_root {
                            is_open.clone().set(true);
                        }
                    }
                } else {
                    invoke(format!("menu_{}", id).as_str(), JsValue::default());
                    is_open.clone().set(false);
                    item_ref.clone().cast::<HtmlElement>().unwrap().blur().unwrap();
                }
            }
            // Escape key
            else if e.key_code() == 27 {
                e.prevent_default();
                e.stop_propagation();
                is_open.clone().set(false);
                if let Some(menu) = menu_ref.cast::<HtmlElement>() {
                    menu.blur().unwrap();
                } else {
                    item_ref.cast::<HtmlElement>().unwrap().blur().unwrap();
                }
            }
        })
    };
    */

    let opened_at_click_time = use_state_eq(|| Some(false));

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
        let is_open = is_open.clone();
        let opened_at_click_time = opened_at_click_time.clone();
        let bar_ref = bar_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if *opened_at_click_time == Some(true) {
                is_open.set(false);
                bar_ref.cast::<HtmlElement>().unwrap().blur().unwrap();
            }
            opened_at_click_time.set(None);
        })
    };

    let onfocus = {
        let menus = menus.clone();
        let opened_at_click_time = opened_at_click_time.clone();
        let selected_item = selected_item.clone();
        let opened_menu = opened_menu.clone();
        Callback::from(move |_: FocusEvent| {
            // Keyboard tab navigation
            if *opened_at_click_time == None {
                if *selected_item == "" {
                    selected_item.set(menus.clone()[0].id.clone());
                    opened_menu.set(String::new());
                } else {
                    opened_menu.set((*selected_item).clone());
                }
            }
        })
    };
    let onfocusout = {
        let is_open = is_open.clone();
        let opened_at_click_time = opened_at_click_time.clone();
        let selected_item = selected_item.clone();
        let opened_menu = opened_menu.clone();
        let is_alt_mode = is_alt_mode.clone(); 
        Callback::from(move |_: _| {
            // Keyboard tab navigation
            if *opened_at_click_time == None {
                is_open.set(false);
                selected_item.set(String::new());
            }
            is_alt_mode.set(false);
            opened_menu.set(String::new());
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
        Callback::from(move |id: String| {
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
            {onmousedown} {onmouseup} {onfocus} {onfocusout}>

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

fn get_previous_item(id: String, brothers_id: &Vec<String>) -> Option<String> {
    let mut previous = None;
    for brother_id in brothers_id {
        if *brother_id == id {
            return previous;
        }
        previous = Some(brother_id.clone());
    }
    None
}
fn get_next_item(id: String, brothers_ids: &Vec<String>) -> Option<String> {
    let mut detected = false;
    for brother_id in brothers_ids {
        if *brother_id == id {
            detected = true
        } else if detected {
            return Some(brother_id.clone());
        }
    }
    None
}

fn is_menu_opened(id: String, opened_menus: UseStateHandle<Vec<String>>) -> bool {
    opened_menus.iter().position(|x| *x == id).is_some()
}
fn open_menu(id: String, brothers: Vec<String>, opened_menus: UseStateHandle<Vec<String>>) {
    if is_menu_opened(id.clone(), opened_menus.clone()) {
        return;
    }

    let mut new_opened_menus = (*opened_menus.clone()).clone();
    brothers.iter().for_each(|brother| {
        if let Some(index) = opened_menus.iter().position(|x| x == brother) {
            new_opened_menus.remove(index);
        }
    });
    new_opened_menus.push(id);
    opened_menus.set(new_opened_menus);
}
fn close_menu(id: String, opened_menus: UseStateHandle<Vec<String>>) {
    remove_id_from_vec_handle(id, opened_menus)
}

fn push_id_to_vec_handle(id: String, vec: UseStateHandle<Vec<String>>) {
    let mut vec_copy = (*vec.clone()).clone();
    vec_copy.push(id);
    vec.set(vec_copy);
}
fn remove_id_from_vec_handle(id: String, vec: UseStateHandle<Vec<String>>) {
    let mut vec_copy = (*vec.clone()).clone();
    let index = vec_copy.iter().position(|x| *x == id);

    if let Some(index) = index {
        vec_copy.remove(index);
        vec.set(vec_copy);
    }
}
