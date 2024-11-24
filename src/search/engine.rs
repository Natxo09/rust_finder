use std::path::PathBuf;
use walkdir::WalkDir;
use crate::utils::file_helpers;

/// Representa un archivo encontrado por el motor de búsqueda.
#[derive(Debug)]
pub struct ArchivoEncontrado {
    pub nombre: String,
    pub ruta: PathBuf,
    pub tamano: u64,  // Tamaño en bytes
    pub fecha: String, // Fecha en formato legible
}

/// Busca archivos en un directorio aplicando filtros.
pub fn buscar_archivos(
    directorio: &str,
    incluir_subdirectorios: bool,
    extensiones: &[String],
    termino_busqueda: &str,
) -> Vec<ArchivoEncontrado> {
    let mut resultados = Vec::new();
    
    if let Ok(ruta_base) = std::path::Path::new(directorio).canonicalize() {
        let walker = WalkDir::new(&ruta_base)
            .follow_links(true)
            .max_depth(if incluir_subdirectorios { std::usize::MAX } else { 1 })
            .into_iter();

        for entrada in walker.filter_entry(|e| !e.file_name().to_string_lossy().starts_with('.'))
                           .filter_map(|e| e.ok()) {
            // Saltamos el directorio base
            if entrada.path() == ruta_base {
                continue;
            }

            let nombre = entrada.file_name().to_string_lossy().to_string();
            
            // Filtrar por término de búsqueda
            if !termino_busqueda.is_empty() && 
               !nombre.to_lowercase().contains(&termino_busqueda.to_lowercase()) {
                continue;
            }

            // Filtrar por extensiones
            if !extensiones.is_empty() {
                let es_valido = entrada.path().is_dir() || 
                    extensiones.iter().any(|ext| 
                        entrada.path()
                            .extension()
                            .and_then(|e| e.to_str())
                            .map_or(false, |e| format!(".{}", e).eq_ignore_ascii_case(ext))
                    );
                
                if !es_valido {
                    continue;
                }
            }

            // Obtener metadatos del archivo
            let tamano = entrada.metadata()
                .map(|m| m.len())
                .unwrap_or(0);

            let fecha = file_helpers::obtener_fecha_modificacion(entrada.path());

            resultados.push(ArchivoEncontrado {
                nombre,
                ruta: entrada.path().to_path_buf(),
                tamano,
                fecha,
            });
        }
    }
    
    resultados
}
