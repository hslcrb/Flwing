use egui::{Painter, Pos2, Rect, Stroke, Color32, Shape as EguiShape, Mesh, epaint};
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator,
    VertexBuffers, FillVertex, StrokeVertex,
};
use lyon::math::point;

use crate::core::document::{ArtBoard, Object, ObjectContent, Layer};
use crate::core::gradient::{Fill, LinearGradient, RadialGradient, MeshGradient};
use crate::core::path::BezierPath;
use crate::core::shape::ShapeKind;
use crate::core::types::{Color, StrokeStyle, Transform2D};

/// 캔버스 상태
pub struct CanvasState {
    /// 뷰 오프셋 (패닝)
    pub offset: egui::Vec2,
    /// 뷰 줌
    pub zoom: f32,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            offset: egui::Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl CanvasState {
    /// 캔버스 좌표 → 화면 좌표
    pub fn canvas_to_screen(&self, x: f64, y: f64) -> Pos2 {
        Pos2::new(
            x as f32 * self.zoom + self.offset.x,
            y as f32 * self.zoom + self.offset.y,
        )
    }

    /// 화면 좌표 → 캔버스 좌표
    pub fn screen_to_canvas(&self, pos: Pos2) -> (f64, f64) {
        (
            ((pos.x - self.offset.x) / self.zoom) as f64,
            ((pos.y - self.offset.y) / self.zoom) as f64,
        )
    }
}

/// 아트보드를 egui Painter에 렌더링
pub fn render_artboard(
    painter: &Painter,
    artboard: &ArtBoard,
    canvas: &CanvasState,
    canvas_rect: Rect,
) {
    // 아트보드 배경
    let top_left = canvas.canvas_to_screen(0.0, 0.0);
    let bottom_right = canvas.canvas_to_screen(artboard.width, artboard.height);
    let ab_rect = Rect::from_min_max(top_left, bottom_right);

    // 배경 그리기
    painter.rect_filled(ab_rect, 0.0, artboard.background.to_egui());
    painter.rect_stroke(ab_rect, 0.0, Stroke::new(1.0, Color32::GRAY), egui::StrokeKind::Outside);

    // 레이어 순서대로 렌더링
    for layer in &artboard.layers {
        if !layer.visible {
            continue;
        }
        render_layer(painter, layer, canvas, canvas_rect);
    }
}

fn render_layer(
    painter: &Painter,
    layer: &Layer,
    canvas: &CanvasState,
    canvas_rect: Rect,
) {
    for obj in &layer.objects {
        if !obj.visible {
            continue;
        }
        render_object(painter, obj, canvas, canvas_rect);
    }
}

fn render_object(
    painter: &Painter,
    obj: &Object,
    canvas: &CanvasState,
    _canvas_rect: Rect,
) {
    match &obj.content {
        ObjectContent::Shape(shape_kind) => {
            let bezier_path = shape_kind.to_path();
            render_path(painter, &bezier_path, &obj.fill, obj.stroke.as_ref(), canvas, &obj.transform);
        }
        ObjectContent::Text(text_data) => {
            // 텍스트 렌더링은 egui 기본 텍스트로 폴백
            let pos = canvas.canvas_to_screen(obj.transform.tx, obj.transform.ty);
            let color = match &obj.fill {
                Fill::Solid(c) => c.to_egui(),
                _ => Color32::BLACK,
            };
            painter.text(
                pos,
                egui::Align2::LEFT_TOP,
                &text_data.content,
                egui::FontId::proportional(text_data.font_size as f32 * canvas.zoom),
                color,
            );
        }
        ObjectContent::Image(_) => {
            // 이미지 렌더링 — 플레이스홀더
            let pos = canvas.canvas_to_screen(obj.transform.tx, obj.transform.ty);
            painter.text(
                pos,
                egui::Align2::LEFT_TOP,
                "🖼 Image",
                egui::FontId::proportional(14.0),
                Color32::GRAY,
            );
        }
        ObjectContent::Group(_) => {
            // 그룹은 재귀적으로 처리 (현재 ID 기반 참조는 미구현)
        }
    }
}

/// BezierPath를 렌더링
fn render_path(
    painter: &Painter,
    path: &BezierPath,
    fill: &Fill,
    stroke: Option<&StrokeStyle>,
    canvas: &CanvasState,
    transform: &Transform2D,
) {
    let lyon_path = path.to_lyon_path();

    // 채우기 렌더링
    match fill {
        Fill::None => {}
        Fill::Solid(color) => {
            fill_path_solid(painter, &lyon_path, color, canvas, transform);
        }
        Fill::Linear(gradient) => {
            fill_path_gradient(painter, &lyon_path, gradient, canvas, transform);
        }
        Fill::Radial(gradient) => {
            fill_path_radial(painter, &lyon_path, gradient, canvas, transform);
        }
        Fill::Mesh(mesh) => {
            fill_path_mesh(painter, &lyon_path, mesh, canvas, transform);
        }
    }

    // 스트로크 렌더링
    if let Some(stroke_style) = stroke {
        stroke_path(painter, &lyon_path, stroke_style, canvas, transform);
    }
}

/// 단색 채우기
fn fill_path_solid(
    painter: &Painter,
    lyon_path: &lyon::path::Path,
    color: &Color,
    canvas: &CanvasState,
    transform: &Transform2D,
) {
    let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();

    let result = tessellator.tessellate_path(
        lyon_path,
        &FillOptions::default(),
        &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
            [vertex.position().x, vertex.position().y]
        }),
    );

    if result.is_err() {
        return;
    }

    let egui_color = color.to_egui();
    let mut mesh = Mesh::default();

    for pos in &geometry.vertices {
        let transformed = transform.apply(crate::core::types::Point2D::new(pos[0] as f64, pos[1] as f64));
        let screen_pos = canvas.canvas_to_screen(transformed.x, transformed.y);
        mesh.vertices.push(epaint::Vertex {
            pos: screen_pos,
            uv: epaint::WHITE_UV,
            color: egui_color,
        });
    }

    for idx in &geometry.indices {
        mesh.indices.push(*idx);
    }

    painter.add(EguiShape::mesh(mesh));
}

/// 선형 그라디언트 채우기
fn fill_path_gradient(
    painter: &Painter,
    lyon_path: &lyon::path::Path,
    gradient: &LinearGradient,
    canvas: &CanvasState,
    transform: &Transform2D,
) {
    let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();

    let result = tessellator.tessellate_path(
        lyon_path,
        &FillOptions::default(),
        &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
            [vertex.position().x, vertex.position().y]
        }),
    );

    if result.is_err() {
        return;
    }

    // 그라디언트 방향 벡터
    let dx = (gradient.end.x - gradient.start.x) as f32;
    let dy = (gradient.end.y - gradient.start.y) as f32;
    let len_sq = dx * dx + dy * dy;

    let mut mesh = Mesh::default();

    for pos in &geometry.vertices {
        let transformed = transform.apply(crate::core::types::Point2D::new(pos[0] as f64, pos[1] as f64));
        let screen_pos = canvas.canvas_to_screen(transformed.x, transformed.y);

        // 그라디언트 t 계산 (시작점으로부터의 투영)
        let px = pos[0] - gradient.start.x as f32;
        let py = pos[1] - gradient.start.y as f32;
        let t = if len_sq > 0.0 {
            ((px * dx + py * dy) / len_sq).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let color = gradient.color_at(t);

        mesh.vertices.push(epaint::Vertex {
            pos: screen_pos,
            uv: epaint::WHITE_UV,
            color: color.to_egui(),
        });
    }

    for idx in &geometry.indices {
        mesh.indices.push(*idx);
    }

    painter.add(EguiShape::mesh(mesh));
}

/// 방사형 그라디언트 채우기
fn fill_path_radial(
    painter: &Painter,
    lyon_path: &lyon::path::Path,
    gradient: &RadialGradient,
    canvas: &CanvasState,
    transform: &Transform2D,
) {
    let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();

    let result = tessellator.tessellate_path(
        lyon_path,
        &FillOptions::default(),
        &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
            [vertex.position().x, vertex.position().y]
        }),
    );

    if result.is_err() {
        return;
    }

    let radius = gradient.radius as f32;
    let cx = gradient.center.x as f32;
    let cy = gradient.center.y as f32;

    let mut mesh = Mesh::default();

    for pos in &geometry.vertices {
        let transformed = transform.apply(crate::core::types::Point2D::new(pos[0] as f64, pos[1] as f64));
        let screen_pos = canvas.canvas_to_screen(transformed.x, transformed.y);

        let dx = pos[0] - cx;
        let dy = pos[1] - cy;
        let dist = (dx * dx + dy * dy).sqrt();
        let t = if radius > 0.0 {
            (dist / radius).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let color = gradient.color_at(t);

        mesh.vertices.push(epaint::Vertex {
            pos: screen_pos,
            uv: epaint::WHITE_UV,
            color: color.to_egui(),
        });
    }

    for idx in &geometry.indices {
        mesh.indices.push(*idx);
    }

    painter.add(EguiShape::mesh(mesh));
}

/// 메쉬 그라디언트 채우기
fn fill_path_mesh(
    painter: &Painter,
    lyon_path: &lyon::path::Path,
    mesh_grad: &MeshGradient,
    canvas: &CanvasState,
    transform: &Transform2D,
) {
    let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();

    let result = tessellator.tessellate_path(
        lyon_path,
        &FillOptions::default(),
        &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
            [vertex.position().x, vertex.position().y]
        }),
    );

    if result.is_err() {
        return;
    }

    // 바운딩 박스 계산
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;
    for pos in &geometry.vertices {
        min_x = min_x.min(pos[0]);
        min_y = min_y.min(pos[1]);
        max_x = max_x.max(pos[0]);
        max_y = max_y.max(pos[1]);
    }

    let w = max_x - min_x;
    let h = max_y - min_y;

    let mut mesh = Mesh::default();

    for pos in &geometry.vertices {
        let transformed = transform.apply(crate::core::types::Point2D::new(pos[0] as f64, pos[1] as f64));
        let screen_pos = canvas.canvas_to_screen(transformed.x, transformed.y);

        let u = if w > 0.0 { (pos[0] - min_x) / w } else { 0.0 };
        let v = if h > 0.0 { (pos[1] - min_y) / h } else { 0.0 };
        let color = mesh_grad.color_at(u, v);

        mesh.vertices.push(epaint::Vertex {
            pos: screen_pos,
            uv: epaint::WHITE_UV,
            color: color.to_egui(),
        });
    }

    for idx in &geometry.indices {
        mesh.indices.push(*idx);
    }

    painter.add(EguiShape::mesh(mesh));
}

/// 스트로크 렌더링
fn stroke_path(
    painter: &Painter,
    lyon_path: &lyon::path::Path,
    style: &StrokeStyle,
    canvas: &CanvasState,
    transform: &Transform2D,
) {
    let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
    let mut tessellator = StrokeTessellator::new();

    let line_cap = match style.line_cap {
        crate::core::types::LineCap::Butt => lyon::tessellation::LineCap::Butt,
        crate::core::types::LineCap::Round => lyon::tessellation::LineCap::Round,
        crate::core::types::LineCap::Square => lyon::tessellation::LineCap::Square,
    };

    let line_join = match style.line_join {
        crate::core::types::LineJoin::Miter => lyon::tessellation::LineJoin::Miter,
        crate::core::types::LineJoin::Round => lyon::tessellation::LineJoin::Round,
        crate::core::types::LineJoin::Bevel => lyon::tessellation::LineJoin::Bevel,
    };

    let result = tessellator.tessellate_path(
        lyon_path,
        &StrokeOptions::default()
            .with_line_width(style.width as f32 * canvas.zoom)
            .with_line_cap(line_cap)
            .with_line_join(line_join),
        &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| {
            [vertex.position().x, vertex.position().y]
        }),
    );

    if result.is_err() {
        return;
    }

    let egui_color = style.color.to_egui();
    let mut mesh = Mesh::default();

    for pos in &geometry.vertices {
        let transformed = transform.apply(crate::core::types::Point2D::new(pos[0] as f64, pos[1] as f64));
        let screen_pos = canvas.canvas_to_screen(transformed.x, transformed.y);
        mesh.vertices.push(epaint::Vertex {
            pos: screen_pos,
            uv: epaint::WHITE_UV,
            color: egui_color,
        });
    }

    for idx in &geometry.indices {
        mesh.indices.push(*idx);
    }

    painter.add(EguiShape::mesh(mesh));
}
