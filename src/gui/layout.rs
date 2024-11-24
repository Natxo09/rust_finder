
use eframe::egui;
use crate::actions::open_directory;
use crate::search::{directory_handler, engine};
use crate::results::table::{self, Resultado};
use crate::results::actions_column;
use crate::utils::file_helpers;
use crate::search::progress_bar::ProgressTracker;
use std::thread;
use std::sync::{Arc, Mutex};

// Añadir estos enums al principio del archivo
#[derive(Debug, PartialEq, Clone)]
pub enum OrderBy {
    Name,
    Type,
    Size,
    Date,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OrderDirection {
    Ascending,
    Descending,
}

pub struct App {
    pub directorio: String,
    pub incluir_subdirectorios: bool,
    pub solo_archivos: bool,
    pub solo_carpetas: bool,
    pub extensiones: Vec<(String, Vec<(String, bool)>)>,
    pub filtro_personalizado: String,
    pub resultados: Vec<Resultado>,
    pub marcar_todas: bool,
    pub termino_busqueda: String,
    pub pagina_actual: usize,
    pub resultados_por_pagina: usize,
    pub buscando: bool,
    pub progreso: f32,
    pub progress_tracker: ProgressTracker,
    pub resultados_compartidos: Arc<Mutex<Option<Vec<Resultado>>>>,
    pub order_by: OrderBy,
    pub order_direction: OrderDirection,
}

impl Default for App {
    fn default() -> Self {
        Self {
            directorio: std::env::current_dir()
                .map(|path| path.to_string_lossy().to_string())
                .unwrap_or_default(),
            incluir_subdirectorios: false,
            solo_archivos: false,
            solo_carpetas: false,
            filtro_personalizado: String::new(),
            resultados: Vec::new(),
            marcar_todas: true,
            termino_busqueda: String::new(),
            pagina_actual: 0,
            resultados_por_pagina: 100,
            extensiones: vec![
                (
                    "Imágenes".to_string(),
                    vec![
                        (".jpg".to_string(), true),
                        (".jpeg".to_string(), true),
                        (".png".to_string(), true),
                        (".gif".to_string(), true),
                        (".bmp".to_string(), true),
                        (".tiff".to_string(), true),
                        (".webp".to_string(), true),
                        (".svg".to_string(), true),
                        (".ico".to_string(), true),
                        (".raw".to_string(), true),
                        (".psd".to_string(), true),
                        (".ai".to_string(), true),
                        (".heic".to_string(), true),
                    ],
                ),
                (
                    "Documentos".to_string(),
                    vec![
                        (".pdf".to_string(), true),
                        (".doc".to_string(), true),
                        (".docx".to_string(), true),
                        (".txt".to_string(), true),
                        (".rtf".to_string(), true),
                        (".odt".to_string(), true),
                        (".xls".to_string(), true),
                        (".xlsx".to_string(), true),
                        (".csv".to_string(), true),
                        (".ods".to_string(), true),
                        (".ppt".to_string(), true),
                        (".pptx".to_string(), true),
                        (".odp".to_string(), true),
                        (".pages".to_string(), true),
                        (".numbers".to_string(), true),
                        (".key".to_string(), true),
                        (".epub".to_string(), true),
                        (".mobi".to_string(), true),
                    ],
                ),
                (
                    "Videos".to_string(),
                    vec![
                        (".mp4".to_string(), true),
                        (".avi".to_string(), true),
                        (".mkv".to_string(), true),
                        (".mov".to_string(), true),
                        (".wmv".to_string(), true),
                        (".flv".to_string(), true),
                        (".webm".to_string(), true),
                        (".m4v".to_string(), true),
                        (".mpg".to_string(), true),
                        (".mpeg".to_string(), true),
                        (".3gp".to_string(), true),
                        (".vob".to_string(), true),
                    ],
                ),
                (
                    "Audios".to_string(),
                    vec![
                        (".mp3".to_string(), true),
                        (".wav".to_string(), true),
                        (".aac".to_string(), true),
                        (".flac".to_string(), true),
                        (".ogg".to_string(), true),
                        (".wma".to_string(), true),
                        (".m4a".to_string(), true),
                        (".mid".to_string(), true),
                        (".midi".to_string(), true),
                        (".aiff".to_string(), true),
                        (".alac".to_string(), true),
                    ],
                ),
                (
                    "Comprimidos".to_string(),
                    vec![
                        (".zip".to_string(), true),
                        (".rar".to_string(), true),
                        (".7z".to_string(), true),
                        (".tar".to_string(), true),
                        (".gz".to_string(), true),
                        (".bz2".to_string(), true),
                        (".xz".to_string(), true),
                        (".iso".to_string(), true),
                    ],
                ),
                (
                    "Programación".to_string(),
                    vec![
                        (".py".to_string(), true),
                        (".js".to_string(), true),
                        (".html".to_string(), true),
                        (".css".to_string(), true),
                        (".cpp".to_string(), true),
                        (".c".to_string(), true),
                        (".h".to_string(), true),
                        (".java".to_string(), true),
                        (".rs".to_string(), true),
                        (".php".to_string(), true),
                        (".rb".to_string(), true),
                        (".swift".to_string(), true),
                        (".go".to_string(), true),
                        (".sql".to_string(), true),
                        (".json".to_string(), true),
                        (".xml".to_string(), true),
                        (".yml".to_string(), true),
                        (".yaml".to_string(), true),
                        (".toml".to_string(), true),
                    ],
                ),
                (
                    "Diseño".to_string(),
                    vec![
                        (".sketch".to_string(), true),
                        (".fig".to_string(), true),
                        (".xd".to_string(), true),
                        (".ai".to_string(), true),
                        (".eps".to_string(), true),
                        (".indd".to_string(), true),
                        (".blend".to_string(), true),
                        (".fbx".to_string(), true),
                        (".obj".to_string(), true),
                        (".stl".to_string(), true),
                    ],
                ),
                (
                    "Ejecutables".to_string(),
                    vec![
                        (".exe".to_string(), true),
                        (".msi".to_string(), true),
                        (".app".to_string(), true),
                        (".dmg".to_string(), true),
                        (".deb".to_string(), true),
                        (".rpm".to_string(), true),
                        (".apk".to_string(), true),
                        (".bat".to_string(), true),
                        (".sh".to_string(), true),
                    ],
                ),
                (
                    "Otros".to_string(),
                    vec![
                        (".log".to_string(), true),
                        (".cfg".to_string(), true),
                        (".ini".to_string(), true),
                        (".dat".to_string(), true),
                        (".bak".to_string(), true),
                        (".tmp".to_string(), true),
                        (".db".to_string(), true),
                        (".sqlite".to_string(), true),
                    ],
                ),
            ],
            buscando: false,
            progreso: 0.0,
            progress_tracker: ProgressTracker::new(),
            resultados_compartidos: Arc::new(Mutex::new(None)),
            order_by: OrderBy::Name,
            order_direction: OrderDirection::Ascending,
        }
    }
}

impl App {
    fn realizar_busqueda(&mut self) {
        if self.directorio.trim().is_empty() {
            if let Ok(current_dir) = std::env::current_dir() {
                self.directorio = current_dir.to_string_lossy().to_string();
            } else {
                self.buscando = false;
                return;
            }
        }

        if directory_handler::validar_directorio(&self.directorio) {
            self.buscando = true;
            self.progreso = 0.0;
            self.progress_tracker.reset();

            // Clonar los datos necesarios para el hilo
            let directorio = self.directorio.clone();
            let incluir_subdirectorios = self.incluir_subdirectorios;
            let solo_archivos = self.solo_archivos;
            let solo_carpetas = self.solo_carpetas;
            let extensiones_activas = self.extensiones.iter()
                .flat_map(|(_, exts)| exts.iter())
                .filter(|(_, estado)| *estado)
                .map(|(ext, _)| ext.clone())
                .collect::<Vec<_>>();
            let termino_busqueda = self.termino_busqueda.clone();
            let progress_tracker = self.progress_tracker.clone();
            let resultados_compartidos = self.resultados_compartidos.clone();

            thread::spawn(move || {
                let archivos = engine::buscar_archivos(
                    &directorio,
                    incluir_subdirectorios,
                    &extensiones_activas,
                    &termino_busqueda,
                    &progress_tracker,
                );
                
                // Aplicar filtros de tipo
                let resultados = archivos.into_iter()
                    .filter(|archivo| {
                        let es_directorio = archivo.ruta.is_dir();
                        
                        // Si ningún filtro está activo, mostrar todo
                        if !solo_archivos && !solo_carpetas {
                            return true;
                        }
                        
                        // Aplicar filtros según las opciones seleccionadas
                        if solo_archivos {
                            !es_directorio
                        } else if solo_carpetas {
                            es_directorio
                        } else {
                            true
                        }
                    })
                    .map(|archivo| Resultado {
                        nombre: archivo.nombre,
                        ruta: archivo.ruta.clone(),
                        tamano: file_helpers::formatear_tamano(archivo.tamano),
                        fecha: archivo.fecha,
                        tipo: if archivo.ruta.is_dir() {
                            "Carpeta".to_string()
                        } else {
                            archivo.ruta.extension()
                                .and_then(|ext| ext.to_str())
                                .map(|ext| format!(".{}", ext))
                                .unwrap_or_else(|| "Desconocido".to_string())
                        },
                    })
                    .collect::<Vec<_>>();
                
                if let Ok(mut shared_results) = resultados_compartidos.lock() {
                    *shared_results = Some(resultados);
                }
            });
        } else {
            self.buscando = false;
        }
    }

    fn ordenar_resultados(&mut self) {
        self.resultados.sort_by(|a, b| {
            let comparacion = match self.order_by {
                OrderBy::Name => a.nombre.to_lowercase().cmp(&b.nombre.to_lowercase()),
                OrderBy::Type => a.tipo.cmp(&b.tipo),
                OrderBy::Size => {
                    // Convertir tamaños a bytes para comparación
                    let parse_size = |s: &str| -> u64 {
                        let parts: Vec<&str> = s.split_whitespace().collect();
                        if parts.len() != 2 { return 0; }
                        
                        let numero: f64 = parts[0].parse().unwrap_or(0.0);
                        match parts[1] {
                            "B" => numero as u64,
                            "KB" => (numero * 1024.0) as u64,
                            "MB" => (numero * 1024.0 * 1024.0) as u64,
                            "GB" => (numero * 1024.0 * 1024.0 * 1024.0) as u64,
                            "TB" => (numero * 1024.0 * 1024.0 * 1024.0 * 1024.0) as u64,
                            _ => 0,
                        }
                    };
                    
                    parse_size(&a.tamano).cmp(&parse_size(&b.tamano))
                },
                OrderBy::Date => a.fecha.cmp(&b.fecha),
            };

            match self.order_direction {
                OrderDirection::Ascending => comparacion,
                OrderDirection::Descending => comparacion.reverse(),
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("panel_lateral")
            .min_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Filtros");
                ui.separator();

                // Botón para seleccionar directorio
                if ui.button("Seleccionar directorio").clicked() {
                    if let Some(path) = open_directory::seleccionar_directorio() {
                        self.directorio = path.to_string_lossy().to_string();
                    }
                }

                // Campo editable para la ruta
                ui.label("Directorio:");
                ui.text_edit_singleline(&mut self.directorio);

                ui.separator();

                // Checkbox para incluir subdirectorios
                ui.checkbox(&mut self.incluir_subdirectorios, "Incluir subdirectorios");

                // Checkbox para filtrar archivos o carpetas
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.solo_archivos, "Solo archivos");
                    ui.checkbox(&mut self.solo_carpetas, "Solo carpetas");
                });

                ui.separator();

                // Botón global para marcar o desmarcar todas las extensiones
                if ui.button(if self.marcar_todas { "Desmarcar todas" } else { "Marcar todas" }).clicked() {
                    self.marcar_todas = !self.marcar_todas;
                    for (_grupo, extensiones) in &mut self.extensiones {
                        for (_, estado) in extensiones {
                            *estado = self.marcar_todas;
                        }
                    }
                }

                ui.separator();

                // Filtros por tipo de archivo con extensiones individuales en desplegables
                for (grupo, extensiones) in &mut self.extensiones {
                    ui.collapsing(grupo.as_str(), |ui| {
                        for (ext, estado) in extensiones {
                            ui.checkbox(estado, ext.as_str());
                        }
                    });
                }

                // Filtro personalizado
                ui.label("Filtro personalizado (ej: .abc):");
                ui.text_edit_singleline(&mut self.filtro_personalizado);
                if ui.button("Añadir filtro").clicked() && !self.filtro_personalizado.is_empty() {
                    if let Some((_, extensiones)) = self.extensiones.iter_mut().find(|(g, _)| g == "Otros") {
                        extensiones.push((self.filtro_personalizado.clone(), true));
                        self.filtro_personalizado.clear();
                    }
                }

                ui.separator();

                // Añadir un espacio flexible que empuje la marca de agua hacia abajo
                ui.add_space(ui.available_height() - 30.0);

                // Marca de agua al final del panel lateral
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.label("Desarrollado por ");
                    ui.hyperlink_to(
                        egui::RichText::new("Natxo")
                            .weak()
                            .italics()
                            .size(14.0),
                        "https://github.com/Natxo09"
                    );
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // Cabecera con información
                ui.horizontal(|ui| {
                    ui.heading("Buscador de Archivos");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let total_resultados = self.resultados.len();
                        ui.label(format!("Total: {} archivos", total_resultados));
                    });
                });
                ui.separator();

                // Barra de búsqueda y botón
                ui.horizontal(|ui| {
                    ui.label("Buscar:");
                    let response = ui.text_edit_singleline(&mut self.termino_busqueda)
                        .on_hover_text("Presiona Enter para buscar");
                    
                    let boton_buscar = ui.button("🔍 Buscar")
                        .on_hover_text("Buscar archivos");

                    if (boton_buscar.clicked() || 
                        (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) 
                        && !self.buscando 
                    {
                        self.buscando = true;
                        self.progreso = 0.0;
                        self.realizar_busqueda();
                    }
                });

                // Verificar si hay resultados nuevos
                if self.buscando {
                    if let Ok(mut shared_results) = self.resultados_compartidos.lock() {
                        if let Some(nuevos_resultados) = shared_results.take() {
                            self.resultados = nuevos_resultados;
                            self.buscando = false;
                            self.progreso = 1.0;
                        }
                    }
                    
                    // Actualizar la barra de progreso
                    self.progreso = self.progress_tracker.get_progress();
                    
                    // Mostrar la barra de progreso
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Buscando archivos...");
                    });
                    ui.add_space(5.0);
                    
                    ui.add(
                        egui::ProgressBar::new(self.progreso)
                            .show_percentage()
                            .animate(true)
                            .desired_width(ui.available_width())
                    );
                    
                    ctx.request_repaint();
                }

                ui.separator();

                // Añadir controles de ordenamiento antes de la tabla
                ui.horizontal(|ui| {
                    ui.label("Ordenar por:");
                    
                    if ui.button("Nombre").clicked() {
                        if self.order_by == OrderBy::Name {
                            self.order_direction = match self.order_direction {
                                OrderDirection::Ascending => OrderDirection::Descending,
                                OrderDirection::Descending => OrderDirection::Ascending,
                            };
                        } else {
                            self.order_by = OrderBy::Name;
                            self.order_direction = OrderDirection::Ascending;
                        }
                        self.ordenar_resultados();
                    }
                    
                    if ui.button("Tipo").clicked() {
                        if self.order_by == OrderBy::Type {
                            self.order_direction = match self.order_direction {
                                OrderDirection::Ascending => OrderDirection::Descending,
                                OrderDirection::Descending => OrderDirection::Ascending,
                            };
                        } else {
                            self.order_by = OrderBy::Type;
                            self.order_direction = OrderDirection::Ascending;
                        }
                        self.ordenar_resultados();
                    }
                    
                    if ui.button("Tamaño").clicked() {
                        if self.order_by == OrderBy::Size {
                            self.order_direction = match self.order_direction {
                                OrderDirection::Ascending => OrderDirection::Descending,
                                OrderDirection::Descending => OrderDirection::Ascending,
                            };
                        } else {
                            self.order_by = OrderBy::Size;
                            self.order_direction = OrderDirection::Ascending;
                        }
                        self.ordenar_resultados();
                    }
                    
                    if ui.button("Fecha").clicked() {
                        if self.order_by == OrderBy::Date {
                            self.order_direction = match self.order_direction {
                                OrderDirection::Ascending => OrderDirection::Descending,
                                OrderDirection::Descending => OrderDirection::Ascending,
                            };
                        } else {
                            self.order_by = OrderBy::Date;
                            self.order_direction = OrderDirection::Ascending;
                        }
                        self.ordenar_resultados();
                    }

                    // Mostrar indicador de dirección
                    ui.label(match self.order_direction {
                        OrderDirection::Ascending => "↑",
                        OrderDirection::Descending => "↓",
                    });
                });

                // Mostrar resultados paginados
                if !self.resultados.is_empty() {
                    let total_paginas = (self.resultados.len() + self.resultados_por_pagina - 1) 
                        / self.resultados_por_pagina;
                    
                    // Controles de paginación
                    ui.horizontal(|ui| {
                        if ui.button("⏮ Primera").clicked() {
                            self.pagina_actual = 0;
                        }
                        if ui.button("◀ Anterior").clicked() && self.pagina_actual > 0 {
                            self.pagina_actual -= 1;
                        }
                        
                        ui.label(format!("Página {} de {}", self.pagina_actual + 1, total_paginas));
                        
                        if ui.button("Siguiente ▶").clicked() && self.pagina_actual < total_paginas - 1 {
                            self.pagina_actual += 1;
                        }
                        if ui.button("Última ⏭").clicked() {
                            self.pagina_actual = total_paginas - 1;
                        }
                    });

                    // Mostrar resultados de la página actual
                    let inicio = self.pagina_actual * self.resultados_por_pagina;
                    let fin = (inicio + self.resultados_por_pagina).min(self.resultados.len());
                    let resultados_pagina = &self.resultados[inicio..fin];

                    let available_height = ui.available_height() - 10.0;
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .max_height(available_height)
                        .show(ui, |ui| {
                            table::mostrar_tabla(
                                ui,
                                resultados_pagina,
                                |ruta| actions_column::abrir_directorio(ruta),
                                |ruta| actions_column::copiar_ruta(ruta),
                            );
                        });

                    // Información de resultados mostrados
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "Mostrando resultados {} - {} de {}",
                            inicio + 1,
                            fin,
                            self.resultados.len()
                        ));
                    });
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label("No se encontraron resultados");
                        ui.add_space(20.0);
                    });
                }
            });
        });
    }
}

// Función para ejecutar la aplicación
pub fn run_app(options: eframe::NativeOptions) {
    eframe::run_native(
        "Buscador de Archivos",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    );
}
