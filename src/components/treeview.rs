use std::rc::Rc;

use yew::{html, Callback, Component, Context, Html, Properties};

use crate::components::treeitem::{TreeItem, TreeItemData, TreeItemMsg};

#[derive(PartialEq, Properties)]
pub struct TreeViewProps {
    pub items: Vec<Rc<TreeItemData>>,
    pub selected_changed: Callback<Vec<String>>,
    pub selected_path: Rc<Vec<String>>,
}

pub enum TreeViewMsg {
    Select(Vec<String>),
}

pub struct TreeView {}

impl Component for TreeView {
    type Message = TreeViewMsg;
    type Properties = TreeViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        return match msg {
            TreeViewMsg::Select(path) => {
                ctx.props().selected_changed.emit(path);
                true
            }
        };
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let parent_message = ctx.link().callback(|msg: TreeItemMsg| match msg {
            TreeItemMsg::UpdateSelectedPath(path) => TreeViewMsg::Select(path),
            _ => unreachable!(),
        });

        html! {
            <ul class="files-tree root">
                {
                    ctx.props().items.iter().map(|item| {
                        html! {
                            <TreeItem
                                key={item.id.clone()}
                                id={item.id.clone()}
                                name={item.name.clone()}
                                parent_path={Vec::default()}
                                selected_path={Rc::clone(&ctx.props().selected_path)}
                                children={&item.children}
                                parent_message={parent_message.clone()}/>
                        }
                    }).collect::<Html>()
                }
            </ul>
        }
    }
}
