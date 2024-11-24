use eframe::egui::{self, Grid, ScrollArea};
use std::path::PathBuf;
use crate::results::actions_column;

/// Representa un resultado individual en la tabla.
#[derive(Debug)]
pub struct Resultado {
    pub nombre: String,
    pub tipo: String,
    pub tamano: String,
    pub fecha: String,
    pub ruta: PathBuf,
}

/// Muestra los resultados en una tabla con columnas.
/// - `ui`: Referencia a la interfaz gráfica.
/// - `resultados`: Lista de resultados a mostrar.
/// - `acciones`: Callback para manejar acciones como abrir y copiar.
pub fn mostrar_tabla<F1, F2>(
    ui: &mut egui::Ui,
    resultados: &[Resultado],
    on_abrir: F1,
    on_copiar: F2,
) where
    F1: Fn(&PathBuf),
    F2: Fn(&PathBuf),
{
    // Usar todo el ancho disponible
    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .max_height(ui.available_height() - 20.0)
            .show(ui, |ui| {
                Grid::new("resultados_grid")
                    .num_columns(5)
                    .spacing([20.0, 8.0])
                    .min_col_width(ui.available_width() / 6.0) // Distribuir el espacio proporcionalmente
                    .striped(true)
                    .show(ui, |ui| {
                        // Encabezados con estilo
                        ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label("Nombre");
                        });
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label("Tipo");
                        });
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label("Tamaño");
                        });
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label("Fecha");
                        });
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label("Acciones");
                        });
                        ui.end_row();

                        // Restaurar estilo normal para las filas
                        ui.style_mut().override_text_style = None;

                        // Filas de resultados
                        for resultado in resultados {
                            // Nombre con elipsis si es muy largo
                            let nombre_truncado = if resultado.nombre.len() > 40 {
                                format!("{}...", &resultado.nombre[..37])
                            } else {
                                resultado.nombre.clone()
                            };

                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.label(nombre_truncado);
                            });
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.label(&resultado.tipo);
                            });
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.label(&resultado.tamano);
                            });
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.label(&resultado.fecha);
                            });
                            
                            ui.horizontal(|ui| {
                                if ui.button("📂")
                                    .on_hover_text("Abrir archivo")
                                    .clicked() 
                                {
                                    actions_column::abrir_archivo(&resultado.ruta);
                                }
                                ui.add_space(4.0);
                                if ui.button("🗂")
                                    .on_hover_text("Abrir carpeta contenedora")
                                    .clicked() 
                                {
                                    actions_column::abrir_directorio(&resultado.ruta);
                                }
                            });
                            ui.end_row();
                        }
                    });
            });
    });
}
