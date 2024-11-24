use std::fs;
use std::path::PathBuf;

/// Busca archivos en el directorio especificado.
/// Si `incluir_subdirectorios` es `true`, busca recursivamente en todos los subdirectorios.
pub fn buscar_en_directorio(directorio: &str, incluir_subdirectorios: bool) -> Vec<PathBuf> {
    let mut resultados = Vec::new();
    let path = PathBuf::from(directorio);

    if incluir_subdirectorios {
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let ruta = entry.path();
                    if ruta.is_dir() {
                        resultados.extend(buscar_en_directorio(
                            ruta.to_str().unwrap(),
                            incluir_subdirectorios,
                        ));
                    } else {
                        resultados.push(ruta);
                    }
                }
            }
        }
    } else {
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    resultados.push(entry.path());
                }
            }
        }
    }

    resultados
}
