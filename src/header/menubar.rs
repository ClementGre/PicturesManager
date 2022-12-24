use std::{
    sync::{Arc, Mutex},
    vec,
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::window;
use yew::prelude::*;

use crate::{
    invoke,
    utils::{keystroke::KeyStroke, logger::info},
};

fn register_shortcut(
    shortcuts: Arc<Mutex<Vec<(KeyStroke, Callback<MouseEvent>)>>>,
    ks: &str,
    event: Callback<MouseEvent>,
) -> &str {
    shortcuts.lock().unwrap().push((KeyStroke::from(ks), event));
    ks
}

#[function_component]
pub fn MenuBar() -> Html {
    let shortcuts: Arc<Mutex<Vec<(KeyStroke, Callback<MouseEvent>)>>> =
        Arc::from(Mutex::from(vec![]));

    let event_shortcuts = shortcuts.clone();

    let is_open = use_state(|| false);
    let opened = use_state(|| 0);

    // KEYBOARD GLOBAL EVENT

    let keyboard_event = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        event_shortcuts
            .clone()
            .lock()
            .unwrap()
            .iter()
            .for_each(|(ks, event)| {
                if ks.matches(&e) {
                    event.emit(MouseEvent::new("click").unwrap());
                }
            });
    }) as Box<dyn FnMut(_)>);

    let _ = window()
        .unwrap()
        .add_event_listener_with_callback("keydown", keyboard_event.as_ref().unchecked_ref())
        .unwrap();
    keyboard_event.forget(); // Makes a memory leak, but this closure is global and needs to live as long as the window is open

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
        .add_event_listener_with_callback("mousedown", mouse_event.as_ref().unchecked_ref())
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

    // MENU EVENTS

    let on_open_gallery = Callback::from(move |_: MouseEvent| {
        info("open gallery");
        invoke("open_gallery", JsValue::default());
    });
    let on_new_gallery = Callback::from(move |_: MouseEvent| {
        info("open gallery");
        invoke("open_gallery", JsValue::default());
    });

    html! {
        <div class={classes!("windows-menu", if *is_open {Some("opened")} else {None})} onclick={on_bar_click}>
            <div class={classes!("menu", if *opened == 1 {Some("opened")} else {None})} onmouseenter={let opened = opened.clone(); Callback::from(move |_: MouseEvent| { opened.set(1); })}>
                <p>{{"Fichier"}}</p>
                <div class="children-box">
                    <div class="children">
                        <div class="menu-item" onclick={&on_open_gallery}>
                            <p>{{"Ouvrir une galerie"}}</p>
                            <p>{{register_shortcut(shortcuts.clone(), "Ctrl+O", on_open_gallery)}}</p>
                        </div>
                        <div class="menu-item">
                            <p>{{"Nouvelle galerie"}}</p>
                            <p>{{register_shortcut(shortcuts.clone(), "Ctrl+N", on_new_gallery)}}</p>
                        </div>
                        <div class="menu-item">
                            <p>{{"Fermer la fenêtre"}}</p>
                        </div>
                        <div class="menu-item">
                            <p>{{"Quitter"}}</p>
                        </div>
                        <div class="split"/>
                        <div class="menu-item">
                            <p>{{"Paramètres"}}</p>
                        </div>
                        <div class="menu-item">
                            <p>{{"À propos"}}</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class={classes!("menu", if *opened == 2 {Some("opened")} else {None})} onmouseenter={let opened = opened.clone(); Callback::from(move |_: MouseEvent| { opened.set(2); })}>
                <p>{{"Édition"}}</p>
                <div class="children-box">
                    <div class="children">
                        <div class="menu-item">
                            <p>{{"Annuler"}}</p>
                        </div>
                        <div class="menu-item">
                            <p>{{"Rétablir"}}</p>
                        </div>
                        <div class="split"/>
                        <div class="menu-item">
                            <p>{{"Couper"}}</p>
                        </div>
                        <div class="menu-item">
                            <p>{{"Copier"}}</p>
                        </div>
                        <div class="menu-item">
                            <p>{{"Coller"}}</p>
                        </div>
                        <div class="menu-item">
                            <p>{{"Tout sélectionner"}}</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class={classes!("menu", if *opened == 3 {Some("opened")} else {None})} onmouseenter={let opened = opened.clone(); Callback::from(move |_: MouseEvent| { opened.set(3); })}>
                <p>{{"Outils"}}</p>
                <div class="children-box">
                    <div class="children">
                        <div class="menu-item">
                            <p>{{"Actualiser la galerie"}}</p>
                        </div>
                        <div class="menu-item">
                            <p>{{"Corriger données Exif"}}</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
