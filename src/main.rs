mod models;
mod utils;
mod components;
mod views;

use dioxus::desktop::tao::platform::windows::WindowBuilderExtWindows;
use dioxus::prelude::*;
use dioxus::desktop::{Config, WindowBuilder};
use crate::components::sidebar::Sidebar;
use crate::views::*;

const CSS: &str = include_str!("../assets/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Sidebar)]
    #[route("/")]
    Home {},
    #[route("/buscar")] // Aunque no tengan contenido aún, deben existir
    Buscar {},
    #[route("/filtrar")]
    Filtrar {},
    #[route("/agregar")]
    Agregar {},
    #[route("/editar")]
    Editar {},
    #[route("/eliminar")]
    Eliminar {},
}

fn main() {
    // 1. Configuramos la ventana (SIN el menú aquí)
    let window = WindowBuilder::new()
        .with_title("DYFIT Student Manager")
        .with_background_color((17, 24, 39, 255)); // RGBA de gray-900

    // 2. Creamos el config y AQUÍ le quitamos el menú
    let config = Config::default()
        .with_window(window)
        .with_menu(None); // <--- El .with_menu() va pegado al Config, no al WindowBuilder

    // 3. Lanzamos
    LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Asegúrate de que este archivo exista en assets/
        style { "{CSS}" }
        Router::<Route> {}
    }
}