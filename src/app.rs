use eframe::egui;
use uuid::Uuid;
use rfd::FileDialog;
use log::info;

use crate::core::document::{Document, Object, TextData, ImageData};
use crate::core::shape::ShapeKind;
use crate::core::types::{Point2D, Color, StrokeStyle};
use crate::core::gradient::Fill;
use crate::renderer::canvas::{CanvasState, render_artboard};
use crate::text::font_manager::FontManager;
use crate::text::shaping::ShapingParams;
use crate::ui;
use crate::format::flw;
use crate::t;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Select,
    Pen,
    Text,
    Rect,
    Ellipse,
}

pub struct FlwingApp {
    pub document: Document,
    pub canvas_state: CanvasState,
    pub font_manager: FontManager,
    pub active_tool: Tool,
    
    // 현재 선택된 드로잉 설정
    pub current_fill: Fill,
    pub current_stroke_width: f64,
    pub text_params: ShapingParams,
    
    // 상태
    pub selected_object_id: Option<Uuid>,
    pub status_message: String,
}

impl FlwingApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // 폰트 매니저 초기화 (시스템 폰트 스캔)
        let font_manager = FontManager::new();
        
        Self {
            document: Document::new("Untitled"),
            canvas_state: CanvasState::default(),
            font_manager,
            active_tool: Tool::Select,
            current_fill: Fill::Solid(Color::white()),
            current_stroke_width: 1.0,
            text_params: ShapingParams::default(),
            selected_object_id: None,
            status_message: "Welcome to Flwing!".to_string(),
        }
    }

    pub fn new_document(&mut self) {
        self.document = Document::new("Untitled");
        self.selected_object_id = None;
    }

    pub fn open_document(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Flwing", &["flw"])
            .pick_file() 
        {
            let path_str = path.to_string_lossy().to_string();
            match flw::load_from_file(&path_str) {
                Ok(doc) => {
                    self.document = doc;
                    self.status_message = format!("Loaded: {}", path_str);
                    // 폰트 매니저에 임베디드 폰트 로드
                    for font in &self.document.embedded_fonts {
                        self.font_manager.load_from_data(&font.family_name, &font.style, font.data.clone());
                    }
                }
                Err(e) => {
                    self.status_message = format!("Error: {}", e);
                }
            }
        }
    }

    pub fn save_document(&mut self) {
        let path = if let Some(p) = &self.document.file_path {
            p.clone()
        } else {
            if let Some(p) = FileDialog::new()
                .add_filter("Flwing", &["flw"])
                .save_file() 
            {
                p.to_string_lossy().to_string()
            } else {
                return;
            }
        };

        match flw::save_to_file(&self.document, &path) {
            Ok(_) => {
                self.document.file_path = Some(path.clone());
                self.status_message = format!("Saved: {}", path);
            }
            Err(e) => {
                self.status_message = format!("Error: {}", e);
            }
        }
    }

    pub fn create_text_outlines(&mut self) {
        // 선택된 텍스트 오브젝트를 윤곽선(Path)으로 변환하는 기능
        if let Some(id) = self.selected_object_id {
            // 실제 구현 시에는 레이어/오브젝트 검색 필요
            info!("Creating outlines for object: {}", id);
        }
    }

    fn handle_input(&mut self, ui: &egui::Ui, response: &egui::Response) {
        if response.clicked() {
            let (cx, cy) = self.canvas_state.screen_to_canvas(response.interact_pointer_pos().unwrap());
            
            match self.active_tool {
                Tool::Rect => {
                    let mut obj = Object::new_shape("Rectangle", ShapeKind::Rectangle {
                        x: cx, y: cy, width: 100.0, height: 80.0, corner_radius: 0.0
                    });
                    obj.fill = self.current_fill.clone();
                    obj.stroke = Some(StrokeStyle { width: self.current_stroke_width, ..Default::default() });
                    self.document.active_artboard_mut().active_layer_mut().add_object(obj);
                }
                Tool::Ellipse => {
                    let mut obj = Object::new_shape("Ellipse", ShapeKind::Ellipse {
                        center: Point2D::new(cx, cy), radius_x: 50.0, radius_y: 40.0
                    });
                    obj.fill = self.current_fill.clone();
                    obj.stroke = Some(StrokeStyle { width: self.current_stroke_width, ..Default::default() });
                    self.document.active_artboard_mut().active_layer_mut().add_object(obj);
                }
                Tool::Text => {
                    let mut obj = Object::new_text("Text", TextData {
                        content: "Hello Flwing".to_string(),
                        font_size: self.text_params.font_size,
                        tracking: self.text_params.tracking,
                        kerning_enabled: self.text_params.kerning,
                        ..Default::default()
                    });
                    obj.transform.tx = cx;
                    obj.transform.ty = cy;
                    self.document.active_artboard_mut().active_layer_mut().add_object(obj);
                }
                _ => {}
            }
        }

        // 패닝 & 줌
        if ui.input(|i| i.modifiers.command) {
            let zoom_delta = ui.input(|i| i.smooth_scroll_delta.y);
            if zoom_delta != 0.0 {
                self.canvas_state.zoom *= (zoom_delta * 0.001).exp();
            }
        } else if response.dragged_by(egui::PointerButton::Middle) {
            self.canvas_state.offset += response.drag_delta();
        }
    }
}

impl eframe::App for FlwingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::menu::show(ctx, self);
        ui::show_side_panels(ctx, self);

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.available_rect_before_wrap();
            let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());
            
            self.handle_input(ui, &response);
            
            let painter = ui.painter_at(rect);
            render_artboard(&painter, self.document.active_artboard_mut(), &self.canvas_state, rect);
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status_message);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Zoom: {:.1}%", self.canvas_state.zoom * 100.0));
                });
            });
        });
    }
}
