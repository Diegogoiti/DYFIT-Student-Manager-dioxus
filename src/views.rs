// src/views/home.rs
use dioxus::prelude::*;
use crate::models::Alumno;


#[derive(Props, PartialEq, Clone)]
pub struct TablaProps {
    alumnos: Vec<Alumno>,
}

#[component]
pub fn TablaAlumnos(props: TablaProps) -> Element {
    rsx! {
        // Altura fija y overflow-y-auto para que el scroll sea interno
        div { class: "overflow-auto rounded-xl border border-gray-800 bg-gray-900 shadow-xl max-h-[600px]",
            table { class: "w-full border-collapse text-left text-xs md:text-sm",
                thead { 
                    // sticky y top-0 mantienen la fila visible al bajar
                    class: "sticky top-0 z-10 bg-gray-800 text-gray-400 uppercase font-bold tracking-wider shadow-sm",
                    tr {
                        th { class: "px-4 py-3", "Sel." }
                        th { class: "px-4 py-3", "ID" }
                        th { class: "px-4 py-3", "Nombre" }
                        th { class: "px-4 py-3", "Cinta" }
                        th { class: "px-4 py-3", "Edad" }
                        th { class: "px-4 py-3", "F. Nacimiento" }
                        th { class: "px-4 py-3", "Representante" }
                        th { class: "px-4 py-3", "Teléfono" }
                    }
                }
                tbody { class: "divide-y divide-gray-800 text-gray-300",
                    for alumno in props.alumnos.iter() {
                        tr { class: "hover:bg-gray-800/50 transition-colors",
                            td { class: "px-4 py-3",
                                input { 
                                    r#type: "checkbox", 
                                    class: "w-4 h-4 rounded border-gray-700 bg-gray-800 text-blue-600 focus:ring-blue-500",
                                    checked: alumno.seleccionado 
                                }
                            }
                            td { class: "px-4 py-3 font-mono text-gray-500", "#{alumno.id}" }
                            td { class: "px-4 py-3 font-bold text-white", "{alumno.nombre}" }
                            td { class: "px-4 py-3",
                                span { class: "px-2 py-1 rounded bg-gray-700 text-[10px] uppercase font-bold text-gray-300", "{alumno.cinta}" }
                            }
                            td { class: "px-4 py-3", "{alumno.edad}" }
                            td { class: "px-4 py-3", "{alumno.fecha_nacimiento}" }
                            td { class: "px-4 py-3", "{alumno.representante}" }
                            td { class: "px-4 py-3 text-blue-400 font-mono", "{alumno.telefono}" }
                        }
                    }
                }
            }
        }
    }
}


#[component]
pub fn Home() -> Element {
    // Datos de prueba estáticos para ver la tabla funcionando
    let mut lista_alumnos = use_signal(|| vec![
        Alumno {
            id: 1,
            nombre: "Diego ".into(),
            cinta: "Negra".to_string(),
            edad: 17,
            fecha_nacimiento: "2009-02-15".into(),
            representante: "Madre de Diego".into(),
            telefono: "0412-1234567".into(),
            seleccionado: false,
        },
        Alumno {
            id: 2,
            nombre: "Andrés Pérez".into(),
            cinta: "Blanca".to_string(),
            edad: 12,
            fecha_nacimiento: "2014-05-20".into(),
            representante: "Carlos Pérez".into(),
            telefono: "0424-7654321".into(),
            seleccionado: false,
        },
    ]);

    rsx! {
        div { class: "flex flex-col h-full space-y-6 p-4",
            div {
                h2 { class: "text-3xl font-black text-black", "Gestión de Alumnos" }
                p { class: "text-gray-400", "Base de datos local del Dojo" }
            }

            // --- AQUÍ ESTÁ EL CAMBIO CLAVE ---
            // Llamamos al componente reutilizable y le pasamos los datos.
            // Usamos .read().clone() porque la prop espera un Vec<Alumno>, no un Signal.
            TablaAlumnos { alumnos: lista_alumnos.read().clone() }
            
            div { class: "text-gray-500 text-xs",
                "Mostrando {lista_alumnos.read().len()} alumnos registrados"
            }
        }
            
            // Pie de tabla con resumen
            div { class: "text-gray-500 text-xs",
                "Mostrando {lista_alumnos.read().len()} alumnos registrados"
            }
        }
    }


#[component]
pub fn Buscar() -> Element {
    rsx! {
        div { class: "space-y-4",
            h2 { class: "text-3xl font-bold text-gray-800", "Buscar Alumnos" }
            p { class: "text-gray-600", "Aquí podrás buscar alumnos por nombre, ID o curso." }

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