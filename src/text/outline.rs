use crate::core::types::Point2D;
use crate::core::path::{BezierPath, PathSegment};

/// ttf-parser OutlineBuilder 구현 — 글리프를 BezierPath로 변환
pub struct GlyphOutlineBuilder {
    path: BezierPath,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
}

impl GlyphOutlineBuilder {
    pub fn new(scale: f64, offset_x: f64, offset_y: f64) -> Self {
        Self {
            path: BezierPath::new(),
            scale,
            offset_x,
            offset_y,
        }
    }

    pub fn build(self) -> BezierPath {
        self.path
    }

    fn transform(&self, x: f32, y: f32) -> Point2D {
        Point2D::new(
            x as f64 * self.scale + self.offset_x,
            // Y축 반전 (폰트 좌표계는 위가 +)
            -y as f64 * self.scale + self.offset_y,
        )
    }
}

impl ttf_parser::OutlineBuilder for GlyphOutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        let p = self.transform(x, y);
        self.path.move_to(p);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let p = self.transform(x, y);
        self.path.line_to(p);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let ctrl = self.transform(x1, y1);
        let to = self.transform(x, y);
        self.path.quad_to(ctrl, to);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let ctrl1 = self.transform(x1, y1);
        let ctrl2 = self.transform(x2, y2);
        let to = self.transform(x, y);
        self.path.cubic_to(ctrl1, ctrl2, to);
    }

    fn close(&mut self) {
        self.path.close();
    }
}

/// 단일 글리프를 벡터 경로로 변환
pub fn glyph_to_path(
    face: &ttf_parser::Face,
    glyph_id: ttf_parser::GlyphId,
    font_size: f64,
    offset_x: f64,
    offset_y: f64,
) -> Option<BezierPath> {
    let units_per_em = face.units_per_em() as f64;
    let scale = font_size / units_per_em;

    let mut builder = GlyphOutlineBuilder::new(scale, offset_x, offset_y);
    face.outline_glyph(glyph_id, &mut builder)?;
    Some(builder.build())
}

/// 텍스트 문자열을 벡터 윤곽선(BezierPath 리스트)으로 변환
/// "텍스트 윤곽선 만들기" 기능
pub fn text_to_outlines(
    text: &str,
    face: &ttf_parser::Face,
    params: &crate::text::shaping::ShapingParams,
) -> Vec<BezierPath> {
    let shaped_lines = crate::text::shaping::shape_text(text, face, params);
    let mut paths = Vec::new();

    for line in &shaped_lines {
        for glyph in line {
            let glyph_id = ttf_parser::GlyphId(glyph.glyph_id);
            if let Some(path) = glyph_to_path(
                face,
                glyph_id,
                glyph.font_size * params.scale_y,
                glyph.offset.x,
                glyph.offset.y + glyph.font_size,
            ) {
                if !path.segments.is_empty() {
                    paths.push(path);
                }
            }
        }
    }

    paths
}
