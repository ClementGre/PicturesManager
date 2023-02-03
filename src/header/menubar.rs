use crate::{
    header::menu::{get_menus, MenuItem},
    invoke,
    utils::{keystroke::KeyStroke, logger::info},
    header::menu_item_component::MenuItemComponent,
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, HtmlElement};
use yew::prelude::*;
use unidecode::unidecode;

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
    let mut alt_shortcuts = vec![];
    menus.iter().for_each(|menu| {
        let name = menu.name.clone().unwrap_or(String::new());
        let split = name.split("_");
        if split.clone().count() >= 2 {
            alt_shortcuts.push((unidecode(split.skip(1).next().unwrap().to_lowercase().as_str())[..1].to_string(), menu.id.clone()));
        }
    });

    let is_open = use_state_eq(|| false);
    let selected_item = use_state_eq(|| String::new());

    let opened_menu = use_state_eq(|| String::new());
    let bar_ref = use_node_ref();

    // GLOBAL EVENTS
    use_memo(
        |_| {
            let keyboard_event = {
                let is_open = is_open.clone();
                let slected_item = selected_item.clone();
                Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                    shortcuts.clone().iter().for_each(|(ks, id)| {
                        if ks.matches(&e) {
                            invoke(format!("menu_{}", id).as_str(), JsValue::default());
                        }
                    });
                    if e.alt_key() {
                        alt_shortcuts.clone().iter().for_each(|(ks, id)| {
                            if ks == &unidecode(e.key().to_lowercase().as_str())[..1].to_string() {
                                is_open.set(true);
                                slected_item.set(id.clone());
                            }
                        });
                    }
                }) as Box<dyn FnMut(_)>)
            };

            let _ = window()
                .unwrap()
                .add_event_listener_with_callback("keydown", keyboard_event.as_ref().unchecked_ref())
                .unwrap();
            keyboard_event.forget(); // Makes a memory leak, but this closure is global and needs to live as long as the window is open

            let mouse_event = {
                let is_open = is_open.clone();
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
        },
        (),
    );

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

    let on_bar_click = {
        let is_open = is_open.clone();
        let bar_ref = bar_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if *is_open {
                is_open.set(false);
            }else{
                bar_ref.cast::<HtmlElement>().unwrap().focus().unwrap();
                is_open.set(true);
            }
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
            ref={bar_ref.clone()}
            tabindex="0"
            class={classes!("windows-menu", if *is_open.clone() {Some("opened")} else {None})} onclick={on_bar_click}>

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

// #[derive(Clone, Properties, PartialEq)]
// struct MenuContext {
//     is_open: UseStateHandle<bool>,
//     selected_item: UseStateHandle<String>,
// }

// #[derive(Clone, Properties, PartialEq)]
// struct MenuItemProps {
//     item: MenuItem,
//     opened_menu: UseStateHandle<String>,
//     brothers: Vec<String>,
//     is_root: bool,
// }

// #[function_component]
// fn MenuItemComponent(props: &MenuItemProps) -> Html {

//     let ctxt = use_context::<MenuContext>().unwrap();
//     let item: MenuItem = props.item.clone();
//     let opened_menu = props.opened_menu.clone();
//     let brothers = props.brothers.clone();
//     let is_root = props.is_root;

//     let children_opened_menu = use_state(|| String::new());
//     let menu_x = use_state_eq(|| 0);
//     let menu_y = use_state_eq(|| 40);
//     let item_ref = use_node_ref();

//     let on_mouse_enter = {
//         let selected_item = ctxt.selected_item.clone();
//         let opened_menu = opened_menu.clone();
//         let is_open = ctxt.is_open.clone();
//         let is_root = is_root.clone();

//         let is_menu = item.items.is_some();
//         let id = item.id.clone();
//         Callback::from(move |_| {
//             selected_item.set(id.clone());
//             if *is_open {
//                 if is_menu { 
//                     if is_root{
//                         opened_menu.set(id.clone()); 
//                     }else{
//                         let timeout = {
//                             let id = id.clone();
//                             let opened_menu = opened_menu.clone();
//                             let selected_item = selected_item.clone();
//                             Timeout::new(300, move || {
//                                 info(format!("Timer fired: opened_menu = {}, selected_item = {}", *opened_menu.clone(), *selected_item.clone()).as_str());
//                                 if *opened_menu != id && *selected_item == id {
//                                     opened_menu.set(id.clone()); 
//                                 }
//                             })
//                         };
//                         timeout.forget();
//                     }
//                 }else if *(opened_menu.clone()) != "" {
//                     info("mouse enter of a non menu item");
//                     let timeout = {
//                         let opened_menu = opened_menu.clone();
//                         let selected_item = selected_item.clone();
//                         info("Setup timer");
//                         Timeout::new(500, move || {
//                             info("Timer fired");
//                             if opened_menu != selected_item && *selected_item != "" {
//                                 info("Timer fired and remove opened menu");
//                                 opened_menu.set(String::new());
//                             }else{
//                                 info(format!("Timer fired but not remove opened menu because opened_menu = {} and selected_item = {}", *opened_menu.clone(), *selected_item.clone()).as_str());
//                             }
//                         })
//                     };
//                 }
//             }
//         })
//     };
//     let on_mouse_leave = {
//         let selected_item = ctxt.selected_item.clone();
//         let opened_menu = opened_menu.clone();
//         let is_root = is_root.clone();
//         let is_open = ctxt.is_open.clone();
//         let is_menu = item.items.is_some();
//         let id = item.id.clone();

//         Callback::from(move |_| {
//             selected_item.set(String::new());

//             if *is_open && is_menu {
                
//             }

//         })

//     };
    

//     // Menu positionning
//     { 
//         let is_root = is_root.clone();
//         let is_menu = item.items.is_some();
//         let menu_x = menu_x.clone();
//         let menu_y = menu_y.clone();
//         let item_ref = item_ref.clone();
        
//         use_effect(move || {
//             if is_menu {
//                 if let Some(menu) = item_ref.cast::<Element>() {
//                     let rect = menu.get_bounding_client_rect();
//                     if is_root {
//                         menu_x.set(rect.x() as i32);
//                         menu_y.set((rect.y() + rect.height()) as i32);
//                     } else {
//                         menu_x.set((rect.x() + rect.width()) as i32);
//                         menu_y.set(rect.y() as i32);
//                     }
//                 }
//             }
//         });
//     }

//     if let Some(items) = item.items {
//         let brothers = items.iter().map(|menu| menu.id.clone()).collect::<Vec<String>>();

//         html! {
//             <div key={item.id.clone()}
//                 ref={item_ref.clone()}
//                 class={classes!(if !is_root {Some("menu-item")} else {None}, "menu", if *opened_menu == item.id {Some("opened")} else {None}, if *ctxt.selected_item == item.id {Some("selected")} else {None})}
//                 onmouseenter={on_mouse_enter}
//                 onmouseleave={on_mouse_leave}>

//                 <MenuTextComponent text={item.name.clone().unwrap()} />
//                 {
//                     if !is_root {
//                         html! { <div class="menu-arrow"><div></div></div> }
//                     } else {
//                         html! {}
//                     }
//                 }

//                 <div class="children-box"
//                     style={format!("padding: {}px 0 0 {}px;", *menu_y, *menu_x)}>
//                     <div class="children no-scrollbar">
//                         <div class="children-scroll">
//                             {
//                                 items.into_iter().map(|item| {
//                                     html!{
//                                         <MenuItemComponent  item={item} opened_menu={children_opened_menu.clone()} brothers={brothers.clone()} is_root={false} />
//                                     }
//                                 }).collect::<Html>()
//                             }
//                         </div>
//                     </div>
//                 </div>

//             </div>
//         }
//     } else if let Some(name) = item.name {
//         let on_click = {
//             let event = format!("menu_{}", item.id);
//             let is_open = ctxt.is_open.clone();

//             Callback::from(move |_: MouseEvent| {
//                 invoke(event.as_str(), JsValue::default());
//                 is_open.set(false);
//             })
//         };

//         html! {
//             <div key={item.id.clone()}
//                 class={classes!("menu-item", "item", if *ctxt.selected_item == item.id {Some("selected")} else {None})}
//                 ref={item_ref.clone()}
//                 onclick={on_click}
//                 onmouseenter={on_mouse_enter}
//                 onmouseleave={on_mouse_leave}>

//                 <MenuTextComponent text={name} />

//                 {
//                     if item.accelerator.is_some() {
//                         html!{
//                             <p>{{item.accelerator.unwrap()}}</p>
//                         }
//                     }else{
//                         html!{}
//                     }
//                 }
//             </div>
//         }
//     } else {
//         // Separator
//         html! {
//             <div key={item.id.clone()} class="menu-item separator">
//                 <hr />
//             </div>
//         }
//     }
// }




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
        }else if detected {
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