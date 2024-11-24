#![windows_subsystem = "windows"]  // Añade esta línea al principio del archivo

mod gui;
mod actions;
mod filters;
mod results;
mod search;
mod utils;

use egui::{ViewportBuilder, Pos2};

fn main() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])  // Tamaño inicial (1024x768)
            .with_min_inner_size([1100.0, 600.0])  // Tamaño mínimo (800x600)
            .with_position(Pos2::new(300.0, 200.0)),  // Posición inicial de la ventana
        ..Default::default()
    };

    gui::layout::run_app(options);
}
