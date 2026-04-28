// build.rs
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        // res.set_icon("icono.ico");
        res.set("CompanyName", "Diego Goitia");
        res.set("FileDescription", "Karate Student Manager");
        res.set(
            "Comments",
            "https://diegogoitia-dev.onrender.com | diegogoitiazx1@gmail.com",
        );
        res.compile().unwrap();
    }
}
