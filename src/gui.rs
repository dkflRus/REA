//Responsible for default GUI of the REA project

use eframe::egui;
pub fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "app_name",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(GUIrea::default())),
    )
}

/// Master view, controls panels of the window(s)
struct GUIrea {}

/// Panel, shows which info is transmitted in selected pipe
struct DataView {}

/// Panel, graphically shows connections of the R/E/As
struct PipeView {}

/// Panel, shows input of the selected R/E/A
struct InputsView {}

impl eframe::App for GUIrea {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("")
            .resizable(true)
            .show(ctx, |ui| {});
    }
}
