use crate::{
    header::menu::{get_menus, MenuItem},
    invoke,
    utils::{keystroke::KeyStroke, logger::info},
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, Element, HtmlElement};
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

#[function_component]
pub fn MenuBar() -> Html {
    let menus = get_menus();

    let mut shortcuts = vec![];
    register_shortcuts(&menus, &mut shortcuts);

    let is_open = use_state_eq(|| false);
    let selected = use_state_eq(|| "".to_string());

    // GLOBAL EVENTS

    {
        let is_open = is_open.clone();
        use_memo( |_| {

            let keyboard_event = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                shortcuts.clone().iter().for_each(|(ks, id)| {
                    if ks.matches(&e) {
                        invoke(format!("menu_{}", id).as_str(), JsValue::default());
                    }
                });
            }) as Box<dyn FnMut(_)>);

            let _ = window()
                .unwrap()
                .add_event_listener_with_callback("keydown", keyboard_event.as_ref().unchecked_ref())
                .unwrap();
            keyboard_event.forget(); // Makes a memory leak, but this closure is global and needs to live as long as the window is open

            let mouse_event = {
                Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                    let target = e.target().and_then(|div| div.dyn_into::<HtmlElement>().ok());
                    if let Some(div) = target {
                        if !div.class_name().split_whitespace().any(|c| "menu-item" == c || "menu" == c) {
                            is_open.set(false); // Close menu only if the target is not a menu-item
                        }
                    }
                }) as Box<dyn FnMut(_)>)
            };

            let _ = window()
                .unwrap()
                .add_event_listener_with_callback("mousedown", mouse_event.as_ref().unchecked_ref())
                .unwrap();
            mouse_event.forget(); // Makes a memory leak, but this closure is global and needs to live as long as the window is open
            info("forgetting mouse event");
        }, ());
    }

    let on_bar_click = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(!*is_open);
        })
    };

    html! {
        <div class={classes!("windows-menu", if *is_open {Some("opened")} else {None})} onclick={on_bar_click}>
            {
                menus.into_iter().map(|menu| {
                    html!{
                        <MenuItemComponent menu={menu} selected={selected.clone()} is_root={{true}} />
                    }
                }).collect::<Html>()
            }

        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct MenuItemProps {
    menu: MenuItem,
    selected: UseStateHandle<String>,
    is_root: bool,
}

#[function_component]
fn MenuItemComponent(props: &MenuItemProps) -> Html {
    let menu: MenuItem = props.menu.clone();
    let selected = props.selected.clone();
    let is_root = props.is_root;
    let selected_child = use_state_eq(|| "".to_string());
    let menu_x = use_state_eq(|| 0);
    let menu_y = use_state_eq(|| 40);

    let on_mouse_enter = {
        let selected = selected.clone();
        let selected_child = selected_child.clone();
        let id = menu.id.clone();
        Callback::from(move |_: MouseEvent| {
            selected_child.set("".to_string());
            selected.set(id.clone())
        })
    };

    let menu_ref = use_node_ref();
    {
        let menu_x = menu_x.clone();
        let menu_y = menu_y.clone();
        let menu_ref = menu_ref.clone();
        let is_root = is_root.clone();

        use_effect(move || {
            if let Some(menu) = menu_ref.cast::<Element>() {
                let rect = menu.get_bounding_client_rect();
                if is_root {
                    menu_x.set(rect.x() as i32);
                    menu_y.set((rect.y() + rect.height()) as i32);
                } else {
                    menu_x.set((rect.x() + rect.width()) as i32);
                    menu_y.set(rect.y() as i32);
                }
            }
        });
    }

    if let Some(items) = menu.items {
        html! {
            <div key={menu.id.clone()}
                ref={menu_ref.clone()}
                class={classes!("menu", if !is_root {Some("menu-item")} else {None}, if *selected == menu.id {Some("opened")} else {None})}
                onmouseenter={on_mouse_enter}>
                <p>{{menu.name.clone()}}</p>
                {
                    if !is_root {
                        html! {
                            <div class="menu-arrow"><div></div></div>
                        }
                    } else {
                        html! {}
                    }
                }

                <div class="children-box"
                    style={format!("padding: {}px 0 0 {}px;", *menu_y, *menu_x)}>
                    <div class="children no-scrollbar">
                        <div class="children-scroll">
                            {
                                items.into_iter().map(|item| {
                                    html!{
                                        <MenuItemComponent menu={item} selected={selected_child.clone()} is_root={{false}}/>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                </div>

            </div>
        }
    } else if let Some(name) = menu.name {
        html! {
            <div key={menu.id.clone()} class="menu-item item"
                onclick={let event = format!("menu_{}", menu.id); Callback::from(move |_: MouseEvent| { invoke(event.as_str(), JsValue::default()); })}
                onmouseenter={on_mouse_enter}>

                <p>{name}</p>
                {
                    if menu.accelerator.is_some() {
                        html!{
                            <p>{{menu.accelerator.unwrap()}}</p>
                        }
                    }else{
                        html!{}
                    }
                }
            </div>
        }
    } else {
        // Separator
        html! {
            <div key={menu.id.clone()} class="menu-item separator">
                <hr />
            </div>
        }
    }
}
