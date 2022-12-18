mod app;

use app::App;
mod header;
mod leftbar;
mod rightbar;
mod mainpane;
mod utils;

fn main() {
    yew::Renderer::<App>::new().render();
}
