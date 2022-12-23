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
    ks: KeyStroke,
    event: Callback<MouseEvent>,
) -> String {
    shortcuts.lock().unwrap().push((ks.clone(), event));
    ks.to_string()
}

#[function_component]
pub fn MenuBar() -> Html {
    let shortcuts: Arc<Mutex<Vec<(KeyStroke, Callback<MouseEvent>)>>> =
        Arc::from(Mutex::from(vec![]));

    let event_shortcuts = shortcuts.clone();

    let closure = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
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
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget(); // Makes a memory leak, but this closure is global and needs to live as long as the window is open

    let on_open_gallery = Callback::from(move |_: MouseEvent| {
        info("open gallery");
        invoke("open_gallery", JsValue::default());
    });

    html! {
        <div class="windows-menu" data-tauri-drag-region="true">
            <div class="menu">
                <p>{{"Fichier"}}</p>
                <div class="children">
                    <div class="menu-item" onclick={&on_open_gallery}>
                        <p>{{"Ouvrir une galerie"}}</p>
                        <p>{{register_shortcut(shortcuts.clone(), KeyStroke::from("shortcut+o"), on_open_gallery)}}</p>

                    </div>
                    <div class="menu-item">
                        <p>{{"Nouvelle galerie"}}</p>
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
            <div class="menu">
                <p>{{"Édition"}}</p>
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
            <div class="menu">
                <p>{{"Outils"}}</p>
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
    }
}
