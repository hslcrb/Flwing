use egui::Ui;
use crate::app::FlwingApp;
use crate::t;
use crate::i18n::locale::{Language, set_language, get_current_language};

pub fn show(ctx: &egui::Context, app: &mut FlwingApp) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button(t!("file"), |ui| {
                if ui.button(t!("new")).clicked() {
                    app.new_document();
                    ui.close_menu();
                }
                if ui.button(t!("open")).clicked() {
                    app.open_document();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button(t!("save")).clicked() {
                    app.save_document();
                    ui.close_menu();
                }
                if ui.button(t!("exit")).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.menu_button(t!("edit"), |ui| {
                if ui.button("Undo").clicked() { ui.close_menu(); }
                if ui.button("Redo").clicked() { ui.close_menu(); }
            });

            ui.menu_button("Language", |ui| {
                let current = get_current_language();
                if ui.selectable_label(current == Language::Korean, "한국어").clicked() {
                    set_language(Language::Korean);
                    ui.close_menu();
                }
                if ui.selectable_label(current == Language::English, "English").clicked() {
                    set_language(Language::English);
                    ui.close_menu();
                }
            });
        });
    });
}
