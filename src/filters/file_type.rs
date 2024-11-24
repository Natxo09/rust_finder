use std::path::Path;

/// Verifica si el archivo cumple con las extensiones seleccionadas.
/// Retorna `true` si el archivo tiene una de las extensiones seleccionadas.
#[allow(dead_code)]
pub fn filtrar_por_tipo(archivo: &Path, extensiones: &[String]) -> bool {
    if let Some(extension) = archivo.extension() {
        if let Some(ext_str) = extension.to_str() {
            return extensiones.contains(&format!(".{}", ext_str).to_lowercase());
        }
    }
    false
}
