use clipboard::{ClipboardContext, ClipboardProvider};

/// Copia un texto al portapapeles.
/// Retorna `true` si se copia correctamente, `false` en caso de error.
pub fn copiar_al_portapapeles(texto: &str) -> bool {
    let mut clipboard: ClipboardContext = match ClipboardProvider::new() {
        Ok(context) => context,
        Err(e) => {
            eprintln!("Error inicializando el portapapeles: {}", e);
            return false;
        }
    };

    match clipboard.set_contents(texto.to_string()) {
        Ok(_) => {
            println!("Texto copiado al portapapeles: {}", texto);
            true
        }
        Err(e) => {
            eprintln!("Error copiando al portapapeles: {}", e);
            false
        }
    }
}
