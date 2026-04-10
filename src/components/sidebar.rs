use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div { class: "flex h-screen bg-gray-100",
            nav { class: "w-64 bg-gray-900 text-white flex flex-col shadow-xl",
                div { class: "p-6 border-b border-gray-800 flex justify-center",
                    h1 { class: "text-2xl font-bold text-red-500", "DYFIT" }
                }
                div { class: "flex-1 px-4 py-6 space-y-2",
                    SidebarItem { to: Route::Home {}, icon: "📋", label: "Consulta" }
                    SidebarItem { to: Route::Home {}, icon: "🔍", label: "Buscar" }
                    SidebarItem { to: Route::Home {}, icon: "📊", label: "Filtrar" }
                    hr { class: "my-6 border-gray-800" }
                    SidebarItem { to: Route::Home {}, icon: "➕", label: "Agregar" }
                    SidebarItem { to: Route::Home {}, icon: "✏️", label: "Editar" }
                    SidebarItem { to: Route::Home {}, icon: "🗑️", label: "Eliminar" }
                }
            }
            // El "hueco" donde se verán las vistas
            main { class: "flex-1 overflow-y-auto p-8",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
fn SidebarItem(to: Route, icon: &'static str, label: &'static str) -> Element {
    rsx! {
        Link { to, class: "flex items-center space-x-3 p-2 rounded hover:bg-gray-800",
            span { "{icon}" }
            span { "{label}" }
        }
    }
}