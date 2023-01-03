use crate::{
    invoke,
    header::menu::{get_menus, MenuItem},
    utils::{keystroke::KeyStroke},
};
use wasm_bindgen::{JsValue, prelude::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;

fn register_shortcuts(items: &Vec<MenuItem>, shortcuts: &mut Vec<(KeyStroke, String)>){
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

    // KEYBOARD GLOBAL EVENT FOR SHORTCUTS

    let mut shortcuts = vec![];
    register_shortcuts(&menus, &mut shortcuts);

    let keyboard_event = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        shortcuts.clone().iter()
            .for_each(|(ks, id)| {
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


    // OPEN STATE

    let is_open = use_state(|| false);
    let opened = use_state(|| "".to_string());

    // MOUSE GLOBAL EVENT

    let mouse_event = {
        let is_open = is_open.clone();
        Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            if *is_open {
                is_open.set(false);
            }
        }) as Box<dyn FnMut(_)>)
    };

    let _ = window()
        .unwrap()
        .add_event_listener_with_callback("mouseup", mouse_event.as_ref().unchecked_ref())
        .unwrap();
    mouse_event.forget(); // Makes a memory leak, but this closure is global and needs to live as long as the window is open

    let on_bar_click = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            if !*is_open {
                is_open.set(true);
            }
        })
    };

    html! {
        <div class={classes!("windows-menu", if *is_open {Some("opened")} else {None})} onclick={on_bar_click}>
            {
                menus.into_iter().map(|menu| {
                    html!{
                        <MenuItemComponent menu={menu} opened={opened.clone()} />
                    }
                }).collect::<Html>()
            }

        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct MenuItemProps {
    menu: MenuItem,
    opened: UseStateHandle<String>,
}

#[function_component]
fn MenuItemComponent(props: &MenuItemProps) -> Html {
    let menu: MenuItem = props.menu.clone();
    let opened = props.opened.clone();
    let opened_children = use_state(|| "".to_string());

    if let Some(items) = menu.items {
        
        html! {
            <div key={menu.id.clone()}
                class={classes!("menu", if *opened == menu.id {Some("opened")} else {None})}
                onmouseenter={let opened = opened.clone(); let id = menu.id.clone(); Callback::from(move |_: MouseEvent| { opened.set(id.clone()); })}>
                <p>{{menu.name.clone()}}</p>

                <div class="children-box">
                    <div class="children">
                        {
                            items.into_iter().map(|item| {
                                html!{
                                    <MenuItemComponent menu={item} opened={opened_children.clone()} />
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        }
    }else if let Some(name) = menu.name {
        html! {
            <div key={menu.id.clone()} class="menu-item" onclick={let event = format!("menu_{}", menu.id); Callback::from(move |_: MouseEvent| { invoke(event.as_str(), JsValue::default()); })}>
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
    }else{
        // Separator
        html! {
            <div key={menu.id.clone()} class="separator"></div>
        }
    }
}
