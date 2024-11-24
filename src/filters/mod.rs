pub mod file_type;
pub mod recursive_search;

use std::path::PathBuf;

pub use self::{file_type::*, recursive_search::*};

/// Aplica los filtros sobre una lista de archivos.
pub fn aplicar_filtros(
    directorio: &str,
    incluir_subdirectorios: bool,
    extensiones: &[String],
) -> Vec<PathBuf> {
    let archivos = buscar_en_directorio(directorio, incluir_subdirectorios);
    archivos
        .into_iter()
        .filter(|archivo| filtrar_por_tipo(archivo, extensiones))
        .collect()
}
