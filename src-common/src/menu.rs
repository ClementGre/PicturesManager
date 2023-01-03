#[derive(Debug)]
pub struct Menu{
    pub name: String, // If root menu, _ can be used to indicate Alt shortcuts
    pub items: Vec<Box<dyn MenuItemTrait>>,
}
#[derive(Debug, Clone)]
pub struct MenuItem{
    pub id: String,
    pub name: String,
    pub accelerator: String,
}
#[derive(Debug)]
pub struct Spacer;

pub trait MenuItemTrait: std::fmt::Debug {
    //type Item;
    //fn add_item(&mut self, item: Self::Item);
    fn is_menu(&self) -> bool;
}
impl MenuItemTrait for Menu {
    //type Item = Menu;
    fn is_menu(&self) -> bool {
        true
    }  
}
impl MenuItemTrait for MenuItem {
    fn is_menu(&self) -> bool {
        false
    }
}
impl MenuItemTrait for Spacer {
    fn is_menu(&self) -> bool {
        false
    }
}

impl Menu{
    pub fn new() -> Self{
        Self{
            name: "".to_string(),
            items: vec![],
        }
    }
}
impl MenuItem{
    pub fn new(id: String, name: String) -> Self{
        Self{
            id,
            name,
            accelerator: "".to_string(),
        }
    }
    pub fn with_accelerator(&mut self, accelerator: String) -> &mut Self{
        self.accelerator = accelerator;
        self
    }
}
