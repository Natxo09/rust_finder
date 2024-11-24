use native_dialog::FileDialog;
use std::path::PathBuf;

/// Selecciona un directorio utilizando un diálogo nativo.
/// Retorna `Some(PathBuf)` si se selecciona un directorio, `None` si se cancela.
pub fn seleccionar_directorio() -> Option<PathBuf> {
    FileDialog::new()
        .set_location("~")
        .show_open_single_dir()
        .map(|path| {
            if let Some(path) = path {
                println!("Directorio seleccionado: {}", path.display());
                Some(path)
            } else {
                println!("Selección de directorio cancelada");
                None
            }
        })
        .unwrap_or_else(|e| {
            eprintln!("Error al seleccionar el directorio: {}", e);
            None
        })
}
