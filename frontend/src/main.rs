mod app;
mod api;
mod components;
mod models;
mod utils;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
