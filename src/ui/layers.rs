use egui::Ui;
use crate::app::FlwingApp;
use crate::t;

pub fn show(ui: &mut Ui, app: &mut FlwingApp) {
    ui.heading(t!("layers"));
    ui.add_space(4.0);

    let doc = &mut app.document;
    let artboard = doc.active_artboard_mut();

    egui::ScrollArea::vertical().show(ui, |ui| {
        for (i, layer) in artboard.layers.iter_mut().enumerate().rev() {
            ui.horizontal(|ui| {
                ui.checkbox(&mut layer.visible, "👁");
                ui.checkbox(&mut layer.locked, "🔒");
                
                let label = format!("{}: {}", i + 1, layer.name);
                if ui.selectable_label(false, label).clicked() {
                    // 레이어 선택 로직
                }
            });
        }
    });
}
