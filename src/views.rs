// src/views/home.rs
use dioxus::prelude::*;
use crate::components::datatable::DataTable;
use crate::my_app;





#[component]
pub fn Home() -> Element {
    let estado = use_context::<Signal<my_app::MyApp>>();
    

    rsx! {
        div { class: "flex flex-col h-full space-y-4 ",
        
                h2 { class: "text-3xl font-bold text-gray-800 text-center", "Consultar" }
                //p { class: "text-gray-600", "Base de datos local del Dojo" }
            

            // --- AQUÍ ESTÁ EL CAMBIO CLAVE ---
            // Llamamos al componente reutilizable y le pasamos los datos.
            // Usamos .read().clone() porque la prop espera un Vec<Alumno>, no un Signal.
            DataTable { alumnos: estado.read().alumnos.clone() }
            
        
            
            // Pie de tabla con resumen
            div { class: "text-gray-500 text-xs",
                "Mostrando {estado.read().alumnos.len()} alumnos registrados"
            }
        }
    }
}

#[component]
pub fn Buscar() -> Element {
    rsx! {
        div { class: "space-y-4",
            h2 { class: "text-3xl font-bold text-gray-800 text-center", "Buscar Alumnos" }
            //p { class: "text-gray-600", "" }

            // Un pequeño indicador de que la vista cargó
            div { class: "p-10 border-2 border-dashed border-gray-300 rounded-xl text-center",
                "Funcionalidad de búsqueda (Próximamente)"
            }
        }
    }
}

#[component]
pub fn Filtrar() -> Element {
    rsx! {
        div { class: "space-y-4",
            h2 { class: "text-3xl font-bold text-gray-800", "Filtrar Alumnos" }
            p { class: "text-gray-600", "Aquí podrás filtrar alumnos por curso, promedio o estado." }

            // Un pequeño indicador de que la vista cargó
            div { class: "p-10 border-2 border-dashed border-gray-300 rounded-xl text-center",
                "Funcionalidad de filtrado (Próximamente)"
            }
        }
    }
}

#[component]
pub fn Agregar() -> Element {
    rsx! {
        div { class: "space-y-4",
            h2 { class: "text-3xl font-bold text-gray-800", "Agregar Alumno" }
            p { class: "text-gray-600", "Aquí podrás agregar nuevos alumnos al sistema." }

            // Un pequeño indicador de que la vista cargó
            div { class: "p-10 border-2 border-dashed border-gray-300 rounded-xl text-center",
                "Formulario de agregar alumno (Próximamente)"
            }
        }
    }
}   

#[component]
pub fn Editar() -> Element {    
    rsx! {
        div { class: "space-y-4",
            h2 { class: "text-3xl font-bold text-gray-800", "Editar Alumno" }
            p { class: "text-gray-600",
                "Aquí podrás editar la información de los alumnos existentes."
            }

            // Un pequeño indicador de que la vista cargó
            div { class: "p-10 border-2 border-dashed border-gray-300 rounded-xl text-center",
                "Formulario de edición de alumno (Próximamente)"
            }
        }
    }
}

#[component]
pub fn Eliminar() -> Element {
    rsx! {
        div { class: "space-y-4",
            h2 { class: "text-3xl font-bold text-gray-800", "Eliminar Alumno" }
            p { class: "text-gray-600", "Aquí podrás eliminar alumnos del sistema." }

            // Un pequeño indicador de que la vista cargó
            div { class: "p-10 border-2 border-dashed border-gray-300 rounded-xl text-center",
                "Funcionalidad de eliminación de alumno (Próximamente)"
            }
        }
    }
}