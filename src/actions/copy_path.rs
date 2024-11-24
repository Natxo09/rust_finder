use clipboard::{ClipboardContext, ClipboardProvider};

/// Copia una ruta al portapapeles.
/// Retorna `true` si se copia correctamente, `false` si ocurre algún error.
#[allow(dead_code)]
pub fn copiar_ruta(ruta: &str) -> bool {
    let mut clipboard: ClipboardContext = match ClipboardProvider::new() {
        Ok(context) => context,
        Err(e) => {
            eprintln!("Error inicializando el portapapeles: {}", e);
            return false;
        }
    };

    match clipboard.set_contents(ruta.to_string()) {
        Ok(_) => {
            println!("Ruta copiada al portapapeles: {}", ruta);
            true
        }
        Err(e) => {
            eprintln!("Error copiando al portapapeles: {}", e);
            false
        }
    }
}
