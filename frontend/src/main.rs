mod app;
mod api;
mod components;
mod models;
mod utils;
mod pages;

fn main() {
    yew::Renderer::<app::App>::new().render();
}