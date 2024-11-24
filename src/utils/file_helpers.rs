use std::fs;
use std::path::{Path, PathBuf};

/// Obtiene el tamaño de un archivo en formato legible (por ejemplo, KB, MB).
pub fn obtener_tamano_legible(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes < KB {
        format!("{} B", bytes)
    } else if bytes < MB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else if bytes < GB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes < TB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else {
        format!("{:.1} TB", bytes as f64 / TB as f64)
    }
}

/// Obtiene la fecha de modificación de un archivo como un string legible.
pub fn obtener_fecha_modificacion(ruta: &Path) -> String {
    if let Ok(metadata) = ruta.metadata() {
        if let Ok(modified) = metadata.modified() {
            if let Ok(datetime) = modified.elapsed() {
                let seconds = datetime.as_secs();
                let days = seconds / 86400;
                return if days < 1 {
                    "Modificado hoy".to_string()
                } else {
                    format!("Modificado hace {} días", days)
                };
            }
        }
    }
    "Fecha desconocida".to_string()
}

/// Verifica si un archivo tiene una extensión específica.
pub fn tiene_extension(ruta: &Path, extension: &str) -> bool {
    if let Some(ext) = ruta.extension() {
        return ext.to_str().unwrap_or("").eq_ignore_ascii_case(extension);
    }
    false
}

/// Convierte un tamaño en bytes a una cadena legible con la unidad apropiada
pub fn formatear_tamano(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let bytes_f64 = bytes as f64;

    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes_f64 < MB {
        format!("{:.2} KB", bytes_f64 / KB)
    } else if bytes_f64 < GB {
        format!("{:.2} MB", bytes_f64 / MB)
    } else if bytes_f64 < TB {
        format!("{:.2} GB", bytes_f64 / GB)
    } else {
        format!("{:.2} TB", bytes_f64 / TB)
    }
}
