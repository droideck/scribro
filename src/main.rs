mod app;
mod db;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
