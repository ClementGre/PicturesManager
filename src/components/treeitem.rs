use std::rc::Rc;

use web_sys::MouseEvent;
use yew::{classes, html, Callback, Component, Context, Html, Properties};
use yew_icons::{Icon, IconId};

use crate::components::contextmenu::{ContextMenu, MenuItem};

#[derive(PartialEq, Properties)]
pub struct TreeItemData {
    pub id: String,
    pub name: String,
    pub children: Rc<Vec<Rc<TreeItemData>>>,
}

#[derive(PartialEq, Properties)]
pub struct TreeItemProps {
    pub id: String,
    pub name: String,
    pub parent_path: Vec<String>,
    pub selected_path: Rc<Vec<String>>,
    pub children: Rc<Vec<Rc<TreeItemData>>>,
    pub parent_message: Callback<TreeItemMsg>,
}

pub enum TreeItemMsg {
    ToggleOpen,
    Select,
    UpdateSelectedPath(Vec<String>),
    ContextMenu,
}

pub struct TreeItem {
    pub is_open: bool,
    pub path: Vec<String>,
}

impl Component for TreeItem {
    type Message = TreeItemMsg;
    type Properties = TreeItemProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut path = ctx.props().parent_path.clone();
        path.push(ctx.props().id.clone());
        Self { is_open: false, path }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        return match msg {
            TreeItemMsg::ToggleOpen => {
                self.is_open = !self.is_open;
                if !self.is_open && self.is_children_selected(ctx) {
                    ctx.link().send_message(TreeItemMsg::Select);
                    return false;
                }
                true
            }
            TreeItemMsg::Select => {
                ctx.props()
                    .parent_message
                    .emit(TreeItemMsg::UpdateSelectedPath(vec![ctx.props().id.clone()]));
                false
            }
            TreeItemMsg::UpdateSelectedPath(mut path) => {
                path.insert(0, ctx.props().id.clone());
                ctx.props().parent_message.emit(TreeItemMsg::UpdateSelectedPath(path));
                false
            }
            TreeItemMsg::ContextMenu => {
                let mut menu = ContextMenu::new();
                menu.add_item(MenuItem {
                    label: "Open in Finder".to_string(),
                    event: "contex_menu_tree_item_files".to_string(),
                    payload: ctx.props().id.clone(),
                    ..Default::default()
                });
                menu.add_separator();
                menu.add_item(MenuItem {
                    label: "Open in Terminal".to_string(),
                    event: "contex_menu_tree_item_terminal".to_string(),
                    payload: ctx.props().id.clone(),
                    ..Default::default()
                });
                menu.show();
                false
            }
        };
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.is_open && self.is_children_selected(ctx) {
            ctx.link().send_message(TreeItemMsg::ToggleOpen);
        }

        html! {
            <li>
            <div class={classes!(if self.is_selected_exactly(ctx) { Some("selected") } else { None })}>
                {
                    // Open button
                    if ctx.props().children.len() > 0 {
                        html! {
                            <div class={classes!(if self.is_open { Some("opened") } else { None })}
                                onclick={ctx.link().callback(move |_: MouseEvent| TreeItemMsg::ToggleOpen)}>
                                <Icon icon_id={IconId::FontAwesomeSolidAngleRight} />
                            </div>
                        }
                    } else {
                        html! { <div /> }
                    }
                }
                <p onclick={ctx.link().callback(move |_: MouseEvent| TreeItemMsg::Select)}
                    oncontextmenu={ctx.link().callback(move |_: MouseEvent| TreeItemMsg::ContextMenu)}>
                    {&ctx.props().name}
                </p>
            </div>
            {
                // Children
                if self.is_open && ctx.props().children.len() > 0 {
                    html! {
                        <ul class="files-tree" style="margin-left: 20px">
                            {
                                ctx.props().children.iter().map(|tree_item| {
                                    html! {
                                        <TreeItem
                                            key={tree_item.id.clone()}
                                            id={tree_item.id.clone()}
                                            name={tree_item.name.clone()}
                                            parent_path={self.path.clone()}
                                            selected_path={Rc::clone(&ctx.props().selected_path)}
                                            children={Rc::clone(&tree_item.children)}
                                            parent_message={ctx.link().callback(move |msg: TreeItemMsg| msg)}/>
                                    }
                                }).collect::<Html>()
                            }
                        </ul>
                    }
                } else { html! {} }
            }
        </li>
        }
    }
}

impl TreeItem {
    fn is_selected_exactly(&self, ctx: &Context<Self>) -> bool {
        *ctx.props().selected_path == self.path
    }
    fn is_selected(&self, ctx: &Context<Self>) -> bool {
        ctx.props().selected_path.starts_with(&self.path)
    }
    fn is_children_selected(&self, ctx: &Context<Self>) -> bool {
        self.is_selected(ctx) && ctx.props().selected_path.len() != self.path.len()
    }
}
