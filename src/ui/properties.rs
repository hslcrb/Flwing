use egui::Ui;
use crate::app::FlwingApp;
use crate::t;
use crate::core::gradient::Fill;

pub fn show(ui: &mut Ui, app: &mut FlwingApp) {
    ui.heading(t!("properties"));
    ui.add_space(8.0);

    // 선택된 오브젝트가 있을 경우 속성 표시
    // (현재는 간단한 전역 속성 편집 예시)
    
    ui.group(|ui| {
        ui.label(t!("fill"));
        // 색상 피커 등 추가 가능
        if let Fill::Solid(mut color) = app.current_fill.clone() {
            let mut egui_color = color.to_egui();
            if ui.color_edit_button_srgba(&mut egui_color).changed() {
                color.r = egui_color.r() as f32 / 255.0;
                color.g = egui_color.g() as f32 / 255.0;
                color.b = egui_color.b() as f32 / 255.0;
                color.a = egui_color.a() as f32 / 255.0;
                app.current_fill = Fill::Solid(color);
            }
        }
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(t!("stroke"));
        ui.add(egui::Slider::new(&mut app.current_stroke_width, 0.0..=20.0).text(t!("width")));
    });

    ui.add_space(16.0);
    
    // 텍스트 도구 선택 시 텍스트 속성 표시
    if matches!(app.active_tool, crate::app::Tool::Text) {
        ui.heading(t!("text"));
        ui.add(egui::Slider::new(&mut app.text_params.font_size, 8.0..=120.0).text(t!("size")));
        ui.add(egui::Slider::new(&mut app.text_params.tracking, -0.1..=0.5).text(t!("tracking")));
        ui.checkbox(&mut app.text_params.kerning, t!("kerning"));
        
        if ui.button(t!("outline_text")).clicked() {
            app.create_text_outlines();
        }
    }
}
