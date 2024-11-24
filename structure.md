file-searcher/
├── src/
│   ├── actions/
│   │   ├── open_directory.rs         # Abrir directorio en el explorador del sistema.
│   │   ├── copy_path.rs              # Copiar la ruta al portapapeles.
│   │   └── mod.rs                    # Módulo para manejar acciones relacionadas con los resultados.
│   ├── filters/
│   │   ├── file_type.rs              # Lógica para filtrar por tipo de archivo.
│   │   ├── recursive_search.rs       # Manejo de la búsqueda en subdirectorios.
│   │   ├── mod.rs                    # Módulo para gestionar los filtros.
│   ├── gui/
│   │   ├── layout.rs                 # Definición de la estructura GUI: panel lateral, buscador, resultados.
│   │   ├── events.rs                 # Manejo de eventos (clics, búsqueda, etc.).
│   │   ├── mod.rs                    # Módulo para la GUI.
│   ├── results/
│   │   ├── table.rs                  # Construcción de la tabla de resultados.
│   │   ├── actions_column.rs         # Implementación de las acciones en la tabla.
│   │   ├── mod.rs                    # Módulo para gestionar los resultados.
│   ├── search/
│   │   ├── engine.rs                 # Lógica principal del motor de búsqueda.
│   │   ├── directory_handler.rs      # Lógica para manejar directorios seleccionados.
│   │   ├── mod.rs                    # Módulo de búsqueda.
│   ├── utils/
│   │   ├── file_helpers.rs           # Funciones auxiliares (e.g., obtener información de archivos).
│   │   ├── clipboard.rs              # Manejo del portapapeles (para copiar rutas).
│   │   ├── mod.rs                    # Módulo para utilidades generales.
│   ├── main.rs                       # Punto de entrada del programa.
│   └── lib.rs                        # Módulo principal.
├── Cargo.toml                        # Archivo de configuración de dependencias de Rust.
└── README.md                         # Documentación del proyecto.
