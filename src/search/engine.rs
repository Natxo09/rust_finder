use std::path::PathBuf;
use walkdir::WalkDir;
use crate::utils::file_helpers;
use crate::search::progress_bar::ProgressTracker;

/// Representa un archivo encontrado por el motor de búsqueda.
#[derive(Debug)]
pub struct ArchivoEncontrado {
    pub nombre: String,
    pub ruta: PathBuf,
    pub tamano: u64,
    pub fecha: String,
}

/// Busca archivos en un directorio aplicando filtros.
pub fn buscar_archivos(
    directorio: &str,
    incluir_subdirectorios: bool,
    extensiones: &[String],
    termino_busqueda: &str,
    progress: &ProgressTracker,
) -> Vec<ArchivoEncontrado> {
    let mut resultados = Vec::new();
    
    if let Ok(ruta_base) = std::path::Path::new(directorio).canonicalize() {
        println!("Iniciando búsqueda en: {}", ruta_base.display());

        let walker = WalkDir::new(&ruta_base)
            .follow_links(true)
            .max_depth(if incluir_subdirectorios { std::usize::MAX } else { 1 })
            .into_iter()
            .filter_entry(|e| !e.file_name().to_string_lossy().starts_with('.'));

        let entradas: Vec<_> = walker
            .filter_map(|e| e.ok())
            .collect();

        let total = entradas.len() as f32;
        println!("Estableciendo total de archivos: {}", total);
        progress.reset();  // Primero reseteamos
        progress.set_total(total);  // Luego establecemos el total
        println!("Verificando total establecido...");
        let _ = progress.get_progress();  // Verificar que se estableció correctamente

        for (index, entrada) in entradas.into_iter().enumerate() {
            println!("Procesando archivo {}/{}", index + 1, total);
            
            if entrada.path() == ruta_base {
                progress.increment();
                continue;
            }

            let nombre = entrada.file_name().to_string_lossy().to_string();
            
            // Filtrar por término de búsqueda
            if !termino_busqueda.is_empty() && 
               !nombre.to_lowercase().contains(&termino_busqueda.to_lowercase()) {
                progress.increment(); // Importante: incrementar incluso si no coincide
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
                    progress.increment(); // Importante: incrementar incluso si no es válido
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

            progress.increment(); // Incrementar después de procesar el archivo
            
            if index % 10 == 0 { // Cada 10 archivos
                println!("Progreso actual: {:.2}%", progress.get_progress() * 100.0);
            }
        }
    }
    
    resultados
}
