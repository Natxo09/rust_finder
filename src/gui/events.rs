use clipboard::ClipboardProvider;

#[allow(dead_code)]
pub fn abrir_directorio(ruta: &str) {
    if let Err(e) = opener::open(ruta) {
        eprintln!("Error al abrir el directorio: {}", e);
    }
}
#[allow(dead_code)]
pub fn copiar_ruta_al_portapapeles(ruta: &str) {
    if let Err(e) = clipboard::ClipboardContext::new()
        .and_then(|mut clipboard| clipboard.set_contents(ruta.to_string()))
    {
        eprintln!("Error al copiar al portapapeles: {}", e);
    }
}
