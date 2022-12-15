mod app;
use app::App;
mod header;
mod leftbar;
mod rightbar;
mod mainpane;

fn main() {
    yew::Renderer::<App>::new().render();
}
