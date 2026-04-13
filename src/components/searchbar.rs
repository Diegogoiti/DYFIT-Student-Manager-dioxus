use dioxus::prelude::*;
use crate::models::Alumno;


#[derive(Clone, Copy, PartialEq)]
enum SearchParam {
    Nombre,
    Apellido,
    Cedula,
    Grado,
}

#[component]
pub fn SearchBar() -> Element {
    let mut search_text = use_signal(|| "".to_string());
    let mut selected_param = use_signal(|| SearchParam::Nombre);

    rsx! {
        div { class: "flex flex-row space-x-2 p-4 bg-white rounded-xl shadow-md border border-gray-200",
            // Dropdown para el parámetro
            select {
                class: "p-2 rounded bg-gray-50 border border-gray-300 text-gray-700",
                onchange: move |evt| {
                    match evt.value().as_str() {
                        "Nombre" => selected_param.set(SearchParam::Nombre),
                        "Apellido" => selected_param.set(SearchParam::Apellido),
                        "Cedula" => selected_param.set(SearchParam::Cedula),
                        "Grado" => selected_param.set(SearchParam::Grado),
                        _ => ()
                    }
                },
                option { value: "Nombre", "Nombre" }
                option { value: "Apellido", "Apellido" }
                option { value: "Cedula", "Cédula" }
                option { value: "Grado", "Grado" }
            }

            // Input de texto
            input {
                class: "flex-1 p-2 rounded border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none",
                placeholder: "Buscar alumno...",
                value: "{search_text}",
                oninput: move |evt| search_text.set(evt.value()),
            }
        }
    }
}