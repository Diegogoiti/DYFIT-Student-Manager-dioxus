#[derive(PartialEq, Clone)]
pub struct Alumno {
    pub id: i32,
    pub nombre: String,
    pub cinta: String,
    pub edad: i32,
    pub fecha_nacimiento: String,
    pub representante: String,
    pub telefono: String,
    pub seleccionado: bool,
}