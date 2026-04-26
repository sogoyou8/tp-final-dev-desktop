mod model;
mod dao;
mod api;
mod app;

use eframe::NativeOptions;
use crate::app::NotesApp;

fn main() -> eframe::Result {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Gestionnaire de Notes Desktop",
        options,
        Box::new(|cc| {
            // Ici nous initialiserons l'application plus tard
            Ok(Box::new(NotesApp::new(cc)))
        }),
    )
}
