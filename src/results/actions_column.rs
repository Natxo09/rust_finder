use std::path::PathBuf;
use std::process::Command;
use arboard::Clipboard;

/// Abre el archivo con la aplicación predeterminada del sistema
pub fn abrir_archivo(ruta: &PathBuf) {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", &ruta.to_string_lossy()])
            .spawn()
            .map_err(|e| println!("Error al abrir el archivo: {}", e))
            .ok();
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(ruta)
            .spawn()
            .map_err(|e| println!("Error al abrir el archivo: {}", e))
            .ok();
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(ruta)
            .spawn()
            .map_err(|e| println!("Error al abrir el archivo: {}", e))
            .ok();
    }
}

/// Abre el directorio que contiene el archivo
pub fn abrir_directorio(ruta: &PathBuf) {
    if let Some(parent) = ruta.parent() {
        #[cfg(target_os = "windows")]
        {
            Command::new("explorer")
                .arg(parent)
                .spawn()
                .map_err(|e| println!("Error al abrir el directorio: {}", e))
                .ok();
        }

        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(parent)
                .spawn()
                .map_err(|e| println!("Error al abrir el directorio: {}", e))
                .ok();
        }

        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open")
                .arg(parent)
                .spawn()
                .map_err(|e| println!("Error al abrir el directorio: {}", e))
                .ok();
        }
    }
}

/// Maneja la acción de copiar la ruta al portapapeles
pub fn copiar_ruta(ruta: &PathBuf) {
    if let Some(ruta_str) = ruta.to_str() {
        match Clipboard::new().and_then(|mut clipboard| clipboard.set_text(ruta_str.to_string())) {
            Ok(_) => println!("Ruta copiada al portapapeles: {}", ruta_str),
            Err(err) => eprintln!("Error al copiar la ruta al portapapeles: {}", err),
        }
    }
}
