
use std::collections::HashSet;
use crate::models::Alumno; 

pub struct MyApp {
    pub alumnos: Vec<Alumno>,
    pub seleccionados: HashSet<usize>,
}

impl MyApp {
    //debo hacer que aqui haga la llamada a la db para obtener los datos de los alumnos, y luego pasarlos a la tabla
    pub fn new(alumnos: Vec<Alumno>) -> Self {
        Self {
            alumnos,
            seleccionados: HashSet::new(),
        }
    }



    pub fn toggle_seleccion(&mut self, index: usize) {
        if self.seleccionados.contains(&index) {
            self.seleccionados.remove(&index);
        } else {
            self.seleccionados.insert(index);
        }
    }
}