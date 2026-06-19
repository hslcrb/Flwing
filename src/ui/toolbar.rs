use egui::Ui;
use crate::app::{FlwingApp, Tool};
use crate::t;

pub fn show(ui: &mut Ui, app: &mut FlwingApp) {
    ui.heading(t!("tools"));
    ui.add_space(8.0);

    ui.vertical_centered_justified(|ui| {
        ui.selectable_value(&mut app.active_tool, Tool::Select, t!("select"));
        ui.selectable_value(&mut app.active_tool, Tool::Pen, t!("pen"));
        ui.selectable_value(&mut app.active_tool, Tool::Text, t!("text"));
        ui.selectable_value(&mut app.active_tool, Tool::Rect, t!("rect"));
        ui.selectable_value(&mut app.active_tool, Tool::Ellipse, t!("ellipse"));
        
        ui.add_space(16.0);
        
        if ui.button(t!("image_trace")).clicked() {
            // 이미지 트레이스 다이얼로그나 동작 트리거
        }
    });
}
