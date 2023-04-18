use gloo_timers::callback::Timeout;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;
use web_sys::{Element, MouseEvent};
use yew::{classes, html, AttrValue, Callback, Component, Context, Html, NodeRef, Properties};

use crate::{
    invoke,
    utils::logger::{info, tr},
};

use super::{
    menu::MenuItem,
    menubar::{MenuTextComponent, NavigationMessage},
};

pub enum MenuItemMsg {
    FireItem,
    UpdatePosition(i32, i32),
    OpenMenuFromTimeout,
    OpenMenu(bool), // true if select first item
    CloseMenuFromTimeout,
    CloseMenu,
    MouseEnter,
    MouseLeave,
    UpdateChildrenSelectedItem(String),
    UpdateChildrenOpenedMenu(String),
    SelectNext,
    SelectPrevious,
}

#[derive(Clone, Properties, PartialEq)]
pub struct MenuItemProps {
    pub item: MenuItem,
    pub is_root: bool,
    pub is_open: bool,
    pub selected_item: AttrValue,
    pub opened_menu: AttrValue,
    pub brothers: Vec<String>,
    pub update_selected_item: Callback<String>,
    pub update_opened_menu: Callback<String>,
    pub navigation_message: NavigationMessage,
    pub navigation_message_received: Callback<bool>,
    pub send_navigation_message: Callback<NavigationMessage>,
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
                    ctx.link().send_message(MenuItemMsg::OpenMenu(false));
                }
            }
            MenuItemMsg::OpenMenu(select_fist_item) => {
                if ctx.props().is_root && !ctx.props().is_open {
                    return false;
                }
                if select_fist_item {
                    self.children_selected_item = ctx.props().item.items.clone().unwrap()[0].id.clone();
                } else {
                    self.children_selected_item = String::new();
                }
                self.children_opened_menu = String::new();
                ctx.props().update_selected_item.emit(ctx.props().item.id.clone());
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
                        ctx.link().send_message(MenuItemMsg::OpenMenu(false));
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
            MenuItemMsg::SelectNext => {
                if let Some(id) = self.get_next_item_id(ctx.clone()) {
                    self.children_selected_item = String::new();

                    ctx.props().update_selected_item.emit(id.clone());
                    if ctx.props().is_root && !ctx.props().opened_menu.is_empty() {
                        // Update opened menu too if root
                        self.children_opened_menu = String::new();
                        ctx.props().update_opened_menu.emit(id.clone());
                    } else {
                        self.children_opened_menu = String::new();
                    }
                }
            }
            MenuItemMsg::SelectPrevious => {
                if let Some(id) = self.get_previous_item_id(ctx.clone()) {
                    self.children_selected_item = String::new();

                    ctx.props().update_selected_item.emit(id.clone());
                    if ctx.props().is_root && !ctx.props().opened_menu.is_empty() {
                        // Update opened menu too if root
                        self.children_opened_menu = String::new();
                        ctx.props().update_opened_menu.emit(id.clone());
                    } else {
                        self.children_opened_menu = String::new();
                    }
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
            return html! {}; // Useless to render if not open and not root
        }

        if ctx.props().is_root.clone() {
            // ROOT MENU
            // IF (selected OR oppened) AND (not opened OR no children selected)
            if self.is_selected_or_opened(ctx) && (!self.is_opened(ctx) || !self.has_selected_children()) {
                match ctx.props().navigation_message {
                    NavigationMessage::Fire | NavigationMessage::Down => {
                        ctx.link().send_message(MenuItemMsg::OpenMenu(true));
                        ctx.props().navigation_message_received.emit(true);
                        return html! {};
                    }
                    NavigationMessage::Left => {
                        ctx.link().send_message(MenuItemMsg::SelectPrevious);
                        ctx.props().navigation_message_received.emit(true);
                        return html! {};
                    }
                    NavigationMessage::Right => {
                        ctx.link().send_message(MenuItemMsg::SelectNext);
                        ctx.props().navigation_message_received.emit(true);
                        return html! {};
                    }
                    NavigationMessage::Close | NavigationMessage::Up => {
                        ctx.link().send_message(MenuItemMsg::CloseMenu);
                        ctx.props().navigation_message_received.emit(true);
                        return html! {};
                    }
                    _ => {}
                }
            }
            // IF opened AND no children opened
            if self.is_opened(ctx) && !self.has_opened_children() {
                match ctx.props().navigation_message {
                    NavigationMessage::LeftRoot => {
                        ctx.link().send_message(MenuItemMsg::SelectPrevious);
                        ctx.props().navigation_message_received.emit(true);
                        return html! {};
                    }
                    NavigationMessage::RightRoot => {
                        ctx.link().send_message(MenuItemMsg::SelectNext);
                        ctx.props().navigation_message_received.emit(true);
                        return html! {};
                    }
                    NavigationMessage::Close => {
                        ctx.link().send_message(MenuItemMsg::CloseMenu);
                        ctx.props().navigation_message_received.emit(true);
                        return html! {};
                    }
                    _ => {}
                }
            }
            if ctx.props().navigation_message == NavigationMessage::CloseRoot {
                if ctx.props().opened_menu.is_empty() {
                    // If no menu oppened, able to catch this event
                    ctx.props().navigation_message_received.emit(true);
                } else if self.is_opened(ctx) {
                    // Closing menu from menubar.rs
                    ctx.link().send_message(MenuItemMsg::CloseMenu);
                    ctx.props().navigation_message_received.emit(true);
                    return html! {};
                }
            }
        } else {
            // NOT ROOT MENU
            // IF selected AND (not a menu OR menu not opened OR no children selected)
            if self.is_selected(ctx) && (!self.is_menu || !self.is_opened(ctx) || !self.has_selected_children()) {
                match ctx.props().navigation_message {
                    NavigationMessage::Up => {
                        ctx.link().send_message(MenuItemMsg::SelectPrevious);
                        ctx.props().navigation_message_received.emit(true);
                        return html! {};
                    }
                    NavigationMessage::Down => {
                        ctx.link().send_message(MenuItemMsg::SelectNext);
                        ctx.props().navigation_message_received.emit(true);
                        return html! {};
                    }
                    NavigationMessage::Right => {
                        if self.is_menu {
                            ctx.link().send_message(MenuItemMsg::OpenMenu(true));
                            ctx.props().navigation_message_received.emit(true);
                            return html! {};
                        } else {
                            ctx.props().send_navigation_message.emit(NavigationMessage::RightRoot);
                        }
                    }
                    NavigationMessage::Left => {
                        ctx.props().send_navigation_message.emit(NavigationMessage::LeftRoot);
                    }
                    NavigationMessage::Fire => {
                        if self.is_menu {
                            ctx.link().send_message(MenuItemMsg::OpenMenu(true));
                        } else {
                            ctx.link().send_message(MenuItemMsg::FireItem);
                        }
                        ctx.props().navigation_message_received.emit(true);
                        return html! {};
                    }
                    _ => {}
                }
            }
            // IF isMenu AND selected AND opened AND no children opened
            if self.is_menu && self.is_selected(ctx) && self.is_selected(ctx) && !self.has_opened_children() {
                match ctx.props().navigation_message {
                    NavigationMessage::Left => {
                        if ctx.props().opened_menu == ctx.props().item.id {
                            ctx.link().send_message(MenuItemMsg::CloseMenu);
                            ctx.props().navigation_message_received.emit(true);
                            return html! {};
                        }
                    }
                    NavigationMessage::Close => {
                        ctx.link().send_message(MenuItemMsg::CloseMenu);
                        ctx.props().navigation_message_received.emit(true);
                        return html! {};
                    }
                    _ => {}
                }
            }
        }

        let onmousedownup = {
            let is_root = ctx.props().is_root.clone();
            Callback::from(move |e: MouseEvent| {
                if !is_root {
                    let target = e.target().and_then(|div| div.dyn_into::<HtmlElement>().ok());
                    if let Some(div) = target {
                        if div.class_name().split_whitespace().any(|c| "menu" == c) {
                            // Prevent menu closing only if the click were performed on this menu and not on a children item
                            e.stop_propagation()
                        }
                    }
                }
            })
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
            let has_children_opened_menu = self.children_opened_menu != "";
            let is_root = ctx.props().is_root;

            let onclick = ctx.link().callback(|_: MouseEvent| MenuItemMsg::OpenMenu(false));

            html! {
                <div key={ctx.props().item.id.clone()} ref={self.item_ref.clone()}
                    class={classes!(
                        if !is_root {Some("menu-item")} else {None},
                        "menu",
                        if is_opened {Some("opened")} else {None},
                        if is_selected || (!is_root && is_opened && !has_selected_browser && (has_children_selected_item || has_children_opened_menu)) {Some("selected")} else {None}
                    )}
                    onmousedown={onmousedownup.clone()} onmouseup={onmousedownup} {onclick} {onmouseenter} {onmouseleave}>

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
                                                navigation_message={ctx.props().navigation_message.clone()}
                                                navigation_message_received={ctx.props().navigation_message_received.clone()}
                                                send_navigation_message={ctx.props().send_navigation_message.clone()}
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

impl MenuItemComponent {
    pub fn get_previous_item_id(&self, ctx: &Context<Self>) -> Option<String> {
        let pos_opt = ctx.props().brothers.iter().position(|id| *id == ctx.props().item.id);
        if let Some(pos) = pos_opt {
            let mut pos = pos;
            while pos > 0 && ctx.props().brothers[pos - 1].starts_with("separator") {
                pos -= 1
            }
            if pos > 0 {
                return Some(ctx.props().brothers[pos - 1].clone());
            } else {
                return Some(ctx.props().brothers[ctx.props().brothers.len() - 1].clone());
            }
        }
        None
    }
    pub fn get_next_item_id(&self, ctx: &Context<Self>) -> Option<String> {
        let pos_opt = ctx.props().brothers.iter().position(|id| *id == ctx.props().item.id);
        if let Some(pos) = pos_opt {
            let mut pos = pos;
            while pos < ctx.props().brothers.len() - 1 && ctx.props().brothers[pos + 1].starts_with("separator") {
                pos += 1
            }
            if pos < ctx.props().brothers.len() - 1 {
                return Some(ctx.props().brothers[pos + 1].clone());
            } else {
                return Some(ctx.props().brothers[0].clone());
            }
        }
        None
    }
    pub fn is_selected(&self, ctx: &Context<Self>) -> bool {
        ctx.props().selected_item == ctx.props().item.id
    }
    pub fn is_opened(&self, ctx: &Context<Self>) -> bool {
        ctx.props().opened_menu == ctx.props().item.id
    }
    pub fn is_selected_or_opened(&self, ctx: &Context<Self>) -> bool {
        self.is_selected(ctx) || self.is_opened(ctx)
    }
    pub fn has_selected_children(&self) -> bool {
        self.children_selected_item != ""
    }
    pub fn has_opened_children(&self) -> bool {
        self.children_opened_menu != ""
    }
}
