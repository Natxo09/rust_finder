use std::fs;
use std::path::{Path, PathBuf};

/// Verifica si el directorio proporcionado es válido.
/// Retorna `true` si es un directorio válido, `false` en caso contrario.
pub fn validar_directorio(directorio: &str) -> bool {
    let path = Path::new(directorio);
    path.is_dir()
}

/// Devuelve una lista de los subdirectorios inmediatos dentro del directorio proporcionado.
/// Retorna un vector de rutas (`PathBuf`) o un vector vacío si hay algún error.
pub fn listar_subdirectorios(directorio: &str) -> Vec<PathBuf> {
    let mut subdirectorios = Vec::new();
    if let Ok(entries) = fs::read_dir(directorio) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    subdirectorios.push(path);
                }
            }
        }
    }
    subdirectorios
}
