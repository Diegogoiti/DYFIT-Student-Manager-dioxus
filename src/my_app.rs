
use std::collections::HashSet;
use crate::models::{Alumno, Database}; 

#[derive(Clone, Copy, PartialEq)]
pub enum Columnas {
    Id,
    Nombre,
    Edad,
    FechaNacimiento,
    Representante,
    Telefono,

}


///estructura que maneja los datos de los alumnos y la ui en memoria 
/// ademas de almacenar la coneccion de la db
pub struct MyApp {
    pub alumnos: Vec<Alumno>,
    pub seleccionados: HashSet<usize>,
    database: Database,
}

impl MyApp {
    pub fn new() -> Self {
        let database = Database::new("./database/database.db").unwrap();
        let alumnos = database.fetch_all().unwrap();
        Self {
            alumnos: alumnos ,
            seleccionados: HashSet::new(),
            database,
        }
    }


pub fn toggle_seleccion(&mut self, id: usize) {
    if self.seleccionados.contains(&id) {
        self.seleccionados.remove(&id);
    } else {
        self.seleccionados.insert(id);
    }
}


pub fn toggle_all(&mut self) {
    if !self.seleccionados.is_empty() {
        self.seleccionados.clear();
    } else {
        self.seleccionados = self.alumnos.iter().map(|a| a.id).collect();
    }
}

pub fn filtrar_alumnos(&self, col: Columnas, query: &str) -> Vec<Alumno> {
        let q = query.to_lowercase();
        if q.is_empty() {
            return self.alumnos.clone();
        }

        self.alumnos.iter().cloned().filter(|a| {
            match col {
                Columnas::Nombre => a.nombre.to_lowercase().contains(&q),
                Columnas::Id => a.id.to_string().contains(&q),
                Columnas::Representante => a.representante.to_lowercase().contains(&q),
                Columnas::Telefono => a.numero_contacto.contains(&q),
                _ => true
            }
        }).collect()
    }

}