#![windows_subsystem = "windows"]

mod gui;
mod actions;
mod filters;
mod results;
mod search;
mod utils;

use egui::{ViewportBuilder, Pos2, IconData};
use image::ImageReader;
use std::io::Cursor;

fn main() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([1100.0, 600.0])
            .with_position(Pos2::new(300.0, 200.0))
            .with_icon(load_icon().unwrap_or_default()),
        ..Default::default()
    };

    gui::layout::run_app(options);
}

fn load_icon() -> Option<IconData> {
    // Incluir el archivo de icono directamente en el ejecutable
    let icon_bytes = include_bytes!("../assets/icon.ico");
    
    // Convertir el icono a RGBA
    let image = ImageReader::new(Cursor::new(icon_bytes.as_ref()))
        .with_guessed_format().ok()?
        .decode().ok()?;
    
    let rgba = image.into_rgba8();
    let (width, height) = rgba.dimensions();
    
    println!("Icono cargado: {}x{} pixels", width, height);
    
    Some(IconData {
        rgba: rgba.into_raw(),
        width: width as _,
        height: height as _,
    })
}
