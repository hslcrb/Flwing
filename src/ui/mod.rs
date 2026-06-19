pub mod toolbar;
pub mod properties;
pub mod layers;
pub mod menu;

use egui::Ui;
use crate::app::FlwingApp;

pub fn show_side_panels(ctx: &egui::Context, app: &mut FlwingApp) {
    egui::SidePanel::left("toolbar").show(ctx, |ui| {
        toolbar::show(ui, app);
    });

    egui::SidePanel::right("properties").show(ctx, |ui| {
        properties::show(ui, app);
    });

    egui::TopBottomPanel::bottom("layers").resizable(true).default_height(150.0).show(ctx, |ui| {
        layers::show(ui, app);
    });
}
