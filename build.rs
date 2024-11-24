fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");  // Asegúrate de que el icono esté en esta ruta
        res.compile().unwrap();
    }
}