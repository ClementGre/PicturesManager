use gloo_timers::callback::Timeout;
use wasm_bindgen::JsValue;
use web_sys::{Element, MouseEvent};
use yew::{classes, html, Callback, Component, Context, Html, NodeRef, Properties};

use crate::{invoke};

use super::{menu::MenuItem, menubar::MenuTextComponent};

pub enum MenuItemMsg {
    FireItem,
    UpdatePosition(i32, i32),
    OpenMenuFromTimeout,
    OpenMenu,
    CloseMenuFromTimeout,
    CloseMenu,
    MouseEnter,
    MouseLeave,
    UpdateChildrenSelectedItem(String),
    UpdateChildrenOpenedMenu(String),
}

#[derive(Clone, Properties, PartialEq)]
pub struct MenuItemProps {
    pub item: MenuItem,
    pub is_root: bool,
    pub is_open: bool,
    pub selected_item: String,
    pub opened_menu: String,
    pub brothers: Vec<String>,
    pub update_selected_item: Callback<String>,
    pub update_opened_menu: Callback<String>,
}

pub struct MenuItemComponent {
    children_selected_item: String,
    children_opened_menu: String,
    is_menu: bool,
    item_ref: NodeRef,
    menu_x: i32,
    menu_y: i32,
}
impl Component for MenuItemComponent {
    type Message = MenuItemMsg;
    type Properties = MenuItemProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            children_selected_item: String::new(),
            children_opened_menu: String::new(),
            is_menu: ctx.props().item.items.is_some(),
            item_ref: NodeRef::default(),
            menu_x: 0,
            menu_y: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MenuItemMsg::FireItem => {
                invoke(format!("menu_{}", ctx.props().item.id).as_str(), JsValue::default());
                return true;
            }
            MenuItemMsg::UpdatePosition(x, y) => {
                if x != self.menu_x || y != self.menu_y {
                    self.menu_x = x;
                    self.menu_y = y;
                    return true;
                }
            }
            MenuItemMsg::OpenMenuFromTimeout => {
                if ctx.props().selected_item == ctx.props().item.id && ctx.props().opened_menu != ctx.props().item.id {
                    ctx.link().send_message(MenuItemMsg::OpenMenu);
                }
            }
            MenuItemMsg::OpenMenu => {
                self.children_selected_item = String::new();
                self.children_opened_menu = String::new();
                ctx.props().update_opened_menu.emit(ctx.props().item.id.clone());
            }
            MenuItemMsg::CloseMenuFromTimeout => {
                if ctx.props().selected_item != "" && ctx.props().opened_menu != "" && ctx.props().selected_item != ctx.props().opened_menu {
                    ctx.link().send_message(MenuItemMsg::CloseMenu);
                }
            }
            MenuItemMsg::CloseMenu => {
                self.children_selected_item = String::new();
                self.children_opened_menu = String::new();
                ctx.props().update_opened_menu.emit(String::new());
            }
            MenuItemMsg::MouseEnter => {
                if ctx.props().selected_item != ctx.props().item.id {
                    ctx.props().update_selected_item.emit(ctx.props().item.id.clone());

                    // Closing last opened menu
                    if !ctx.props().is_root && ctx.props().opened_menu != "" {
                        let callback = ctx.link().callback(|_| MenuItemMsg::CloseMenuFromTimeout);
                        Timeout::new(500, move || {
                            callback.emit(());
                        })
                        .forget();
                    }
                }
                // Opening this menu
                if self.is_menu && ctx.props().opened_menu != ctx.props().item.id {
                    if ctx.props().is_root {
                        ctx.link().send_message(MenuItemMsg::OpenMenu);
                    } else {
                        let callback = ctx.link().callback(|_| MenuItemMsg::OpenMenuFromTimeout);
                        Timeout::new(200, move || {
                            callback.emit(());
                        })
                        .forget();
                    }
                }
                // Update is done updating the parent state
            }
            MenuItemMsg::MouseLeave => {
                if ctx.props().selected_item != "" {
                    ctx.props().update_selected_item.emit(String::new());
                }
                // Update is done updating the parent state
            }
            MenuItemMsg::UpdateChildrenSelectedItem(id) => {
                if self.children_selected_item != id {
                    self.children_selected_item = id.clone();
                    return true;
                }
            }
            MenuItemMsg::UpdateChildrenOpenedMenu(id) => {
                if self.children_opened_menu != id {
                    self.children_opened_menu = id.clone();
                    return true;
                }
            }
        }
        false
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if ctx.props().is_open {

            // Update menu position
            if self.is_menu {
                if let Some(menu) = self.item_ref.cast::<Element>() {
                    let rect = menu.get_bounding_client_rect();
                    if ctx.props().is_root {
                        ctx.link()
                            .send_message(MenuItemMsg::UpdatePosition(rect.x() as i32, (rect.y() + rect.height()) as i32));
                    } else {
                        ctx.link()
                            .send_message(MenuItemMsg::UpdatePosition((rect.x() + rect.width()) as i32, rect.y() as i32));
                    }
                }
            }

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !ctx.props().is_open && !ctx.props().is_root {
            return html! {}; // Useless to render if not open
        }

        let onmousedownup = {
            let is_root = ctx.props().is_root.clone();
            Callback::from(move |e: MouseEvent| if !is_root { e.stop_propagation() })
        };


        let onmouseenter = ctx.link().callback(move |_: MouseEvent| MenuItemMsg::MouseEnter);
        let onmouseleave = ctx.link().callback(|_: MouseEvent| MenuItemMsg::MouseLeave);

        if let Some(items) = ctx.props().item.items.clone() {
            let update_children_selected_item = ctx.link().callback(|id: String| MenuItemMsg::UpdateChildrenSelectedItem(id));
            let update_children_opened_menu = ctx.link().callback(|id: String| MenuItemMsg::UpdateChildrenOpenedMenu(id));

            let brothers = items.iter().map(|menu| menu.id.clone()).collect::<Vec<String>>();
            let is_opened = ctx.props().opened_menu == ctx.props().item.id;
            let is_selected = ctx.props().selected_item == ctx.props().item.id;
            let has_selected_browser = ctx.props().selected_item != "";
            let has_children_selected_item = self.children_selected_item != "";
            let has_children_opened_menu =  self.children_opened_menu != "";
            let is_root = ctx.props().is_root;
            html! {
                <div key={ctx.props().item.id.clone()} ref={self.item_ref.clone()}
                    class={classes!(
                        if !is_root {Some("menu-item")} else {None},
                        "menu",
                        if is_opened {Some("opened")} else {None},
                        if is_selected || (!is_root && is_opened && !has_selected_browser && (has_children_selected_item || has_children_opened_menu)) {Some("selected")} else {None}
                    )}
                    onmousedown={onmousedownup.clone()} onmouseup={onmousedownup} {onmouseenter} {onmouseleave}>

                    <MenuTextComponent text={ctx.props().item.name.clone().unwrap()} />
                    {
                        if !is_root {
                            html! { <div class="menu-arrow"><div></div></div> }
                        } else {
                            html! {}
                        }
                    }

                    <div class="children-box"
                        style={format!("padding: {}px 0 0 {}px;", self.menu_y, self.menu_x)}>
                        <div class="children no-scrollbar">
                            <div class="children-scroll">
                                {
                                    items.into_iter().map(|item| {
                                        html!{
                                            <MenuItemComponent
                                                item={item}
                                                is_root={false}
                                                is_open={ctx.props().is_open.clone()}
                                                selected_item={self.children_selected_item.clone()}
                                                opened_menu={self.children_opened_menu.clone()}
                                                brothers={brothers.clone()}
                                                update_selected_item={update_children_selected_item.clone()}
                                                update_opened_menu={update_children_opened_menu.clone()}
                                            />
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        </div>
                    </div>

                </div>
            }
        } else if let Some(name) = ctx.props().item.name.clone() {
            let onclick = ctx.link().callback(|_: MouseEvent| MenuItemMsg::FireItem);

            html! {
                <div key={ctx.props().item.id.clone()} ref={self.item_ref.clone()}
                    class={classes!(
                        "menu-item", "item",
                        if *ctx.props().selected_item == ctx.props().item.id {Some("selected")} else {None}
                    )}
                    {onclick} {onmouseenter} {onmouseleave}>

                    <MenuTextComponent text={name} />

                    {
                        if let Some(acc) = ctx.props().item.accelerator.clone() {
                            html!{ <p>{acc}</p> }
                        }else{
                            html!{}
                        }
                    }
                </div>
            }
        } else {
            // Separator
            html! {
                <div key={ctx.props().item.id.clone()} class="menu-item separator">
                    <hr />
                </div>
            }
        }
    }
}
