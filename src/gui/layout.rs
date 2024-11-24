use eframe::egui;
use crate::actions::open_directory;
use crate::search::{directory_handler, engine};
use crate::results::table::{self, Resultado};
use crate::results::actions_column;
use crate::utils::file_helpers;

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
}

impl Default for App {
    fn default() -> Self {
        Self {
            directorio: String::new(),
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
        }
    }
}

impl App {
    fn realizar_busqueda(&mut self) {
        if directory_handler::validar_directorio(&self.directorio) {
            // Recolectar extensiones activas
            let mut extensiones_activas: Vec<String> = Vec::new();
            for (_grupo, extensiones) in &self.extensiones {
                for (ext, estado) in extensiones {
                    if *estado {
                        extensiones_activas.push(ext.clone());
                    }
                }
            }

            // Usar el motor de búsqueda
            let archivos_encontrados = engine::buscar_archivos(
                &self.directorio,
                self.incluir_subdirectorios,
                &extensiones_activas,
                &self.termino_busqueda,
            );

            // Convertir los resultados
            self.resultados = archivos_encontrados
                .into_iter()
                .map(|archivo| Resultado {
                    nombre: archivo.nombre,
                    tipo: if archivo.ruta.is_dir() {
                        "Carpeta".to_string()
                    } else {
                        archivo.ruta.extension()
                            .and_then(|ext| ext.to_str())
                            .map(|ext| format!(".{}", ext))
                            .unwrap_or_else(|| "Desconocido".to_string())
                    },
                    tamano: file_helpers::formatear_tamano(archivo.tamano),
                    fecha: archivo.fecha,
                    ruta: archivo.ruta,
                })
                .collect();

            println!("Encontrados {} archivos con término '{}'", 
                    self.resultados.len(), 
                    self.termino_busqueda);
            
            // Resetear estado de búsqueda
            self.buscando = false;
            self.progreso = 1.0;
        } else {
            println!("Directorio no válido: {}", self.directorio);
            self.buscando = false;
            self.progreso = 0.0;
        }
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

                // Mostrar barra de progreso cuando está buscando
                if self.buscando {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.spinner(); // Muestra un spinner giratorio
                        ui.label("Buscando archivos...");
                    });
                    ui.add_space(5.0);
                    ui.add(egui::ProgressBar::new(self.progreso)
                        .show_percentage()
                        .animate(true));
                }

                ui.separator();

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
pub fn run() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Buscador de Archivos",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    );
}
