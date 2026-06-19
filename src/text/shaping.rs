use crate::core::types::Point2D;

/// 셰이핑된 글리프 — 위치가 계산된 개별 글자
#[derive(Debug, Clone)]
pub struct ShapedGlyph {
    pub glyph_id: u16,
    /// 기준선(baseline)으로부터의 위치
    pub offset: Point2D,
    /// 글리프의 전진 폭
    pub advance: f64,
    /// 폰트 크기 (렌더링용)
    pub font_size: f64,
}

/// 텍스트 셰이핑 파라미터
#[derive(Debug, Clone)]
pub struct ShapingParams {
    pub font_size: f64,
    /// 자간 (tracking) — em 단위 추가 간격
    pub tracking: f64,
    /// 커닝 활성화
    pub kerning: bool,
    /// 종횡비 X
    pub scale_x: f64,
    /// 종횡비 Y
    pub scale_y: f64,
    /// 줄 간격 배율
    pub line_height: f64,
}

impl Default for ShapingParams {
    fn default() -> Self {
        Self {
            font_size: 24.0,
            tracking: 0.0,
            kerning: true,
            scale_x: 1.0,
            scale_y: 1.0,
            line_height: 1.2,
        }
    }
}

/// 텍스트 셰이핑 엔진
/// ttf-parser를 사용하여 글리프 인덱스 + 위치 계산
pub fn shape_text(
    text: &str,
    face: &ttf_parser::Face,
    params: &ShapingParams,
) -> Vec<Vec<ShapedGlyph>> {
    let units_per_em = face.units_per_em() as f64;
    let scale_factor = params.font_size / units_per_em;
    let line_spacing = params.font_size * params.line_height;

    let mut lines: Vec<Vec<ShapedGlyph>> = Vec::new();

    for (line_idx, line) in text.lines().enumerate() {
        let mut glyphs = Vec::new();
        let mut cursor_x = 0.0;
        let cursor_y = line_idx as f64 * line_spacing;
        let mut prev_glyph_id: Option<ttf_parser::GlyphId> = None;

        for ch in line.chars() {
            let glyph_id = face.glyph_index(ch).unwrap_or(ttf_parser::GlyphId(0));

            // 커닝 적용
            if params.kerning {
                if let Some(prev) = prev_glyph_id {
                    if let Some(kern_table) = face.tables().kern {
                        for subtable in kern_table.subtables {
                            if let Some(kern_value) = subtable.glyphs_kerning(prev, glyph_id) {
                                cursor_x += kern_value as f64 * scale_factor;
                            }
                        }
                    }
                }
            }

            // 글리프 전진 폭
            let advance = face
                .glyph_hor_advance(glyph_id)
                .unwrap_or(0) as f64
                * scale_factor
                * params.scale_x;

            glyphs.push(ShapedGlyph {
                glyph_id: glyph_id.0,
                offset: Point2D::new(cursor_x, cursor_y),
                advance,
                font_size: params.font_size,
            });

            cursor_x += advance;

            // 자간 (tracking) 적용
            cursor_x += params.tracking * params.font_size;

            prev_glyph_id = Some(glyph_id);
        }

        lines.push(glyphs);
    }

    lines
}

/// 텍스트 전체 폭 계산
pub fn measure_text_width(
    text: &str,
    face: &ttf_parser::Face,
    params: &ShapingParams,
) -> f64 {
    let shaped = shape_text(text, face, params);
    shaped
        .iter()
        .map(|line| {
            line.last()
                .map(|g| g.offset.x + g.advance)
                .unwrap_or(0.0)
        })
        .fold(0.0_f64, f64::max)
}
