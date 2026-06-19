use serde::{Deserialize, Serialize};
use crate::core::types::{Point2D, Color};

/// 색상 정지점
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorStop {
    /// 0.0 ~ 1.0 사이의 위치
    pub offset: f32,
    pub color: Color,
}

/// 선형 그라디언트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearGradient {
    pub start: Point2D,
    pub end: Point2D,
    pub stops: Vec<ColorStop>,
}

impl LinearGradient {
    pub fn new(start: Point2D, end: Point2D) -> Self {
        Self {
            start,
            end,
            stops: vec![
                ColorStop { offset: 0.0, color: Color::black() },
                ColorStop { offset: 1.0, color: Color::white() },
            ],
        }
    }

    /// 주어진 비율 t (0.0~1.0)에서의 보간 색상
    pub fn color_at(&self, t: f32) -> Color {
        if self.stops.is_empty() {
            return Color::black();
        }
        if t <= self.stops[0].offset {
            return self.stops[0].color;
        }
        if t >= self.stops.last().unwrap().offset {
            return self.stops.last().unwrap().color;
        }

        for i in 0..self.stops.len() - 1 {
            let s0 = &self.stops[i];
            let s1 = &self.stops[i + 1];
            if t >= s0.offset && t <= s1.offset {
                let local_t = (t - s0.offset) / (s1.offset - s0.offset);
                return s0.color.lerp(&s1.color, local_t);
            }
        }
        self.stops.last().unwrap().color
    }
}

/// 방사형 그라디언트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadialGradient {
    pub center: Point2D,
    pub radius: f64,
    pub focal_point: Point2D,
    pub stops: Vec<ColorStop>,
}

impl RadialGradient {
    pub fn new(center: Point2D, radius: f64) -> Self {
        Self {
            center,
            radius,
            focal_point: center,
            stops: vec![
                ColorStop { offset: 0.0, color: Color::white() },
                ColorStop { offset: 1.0, color: Color::black() },
            ],
        }
    }

    pub fn color_at(&self, t: f32) -> Color {
        if self.stops.is_empty() {
            return Color::black();
        }
        let t = t.clamp(0.0, 1.0);
        for i in 0..self.stops.len() - 1 {
            let s0 = &self.stops[i];
            let s1 = &self.stops[i + 1];
            if t >= s0.offset && t <= s1.offset {
                let local_t = if (s1.offset - s0.offset).abs() < f32::EPSILON {
                    0.0
                } else {
                    (t - s0.offset) / (s1.offset - s0.offset)
                };
                return s0.color.lerp(&s1.color, local_t);
            }
        }
        self.stops.last().unwrap().color
    }
}

/// 메쉬 그라디언트 제어점
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshPoint {
    pub position: Point2D,
    pub color: Color,
    /// 각 방향으로의 핸들 (접선)
    pub handle_top: Point2D,
    pub handle_bottom: Point2D,
    pub handle_left: Point2D,
    pub handle_right: Point2D,
}

/// 자유 메쉬 그라디언트 — 격자 기반
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshGradient {
    pub rows: u32,
    pub cols: u32,
    pub points: Vec<Vec<MeshPoint>>,
}

impl MeshGradient {
    /// 새 메쉬 그라디언트 생성 (rows×cols 격자)
    pub fn new(x: f64, y: f64, width: f64, height: f64, rows: u32, cols: u32) -> Self {
        let mut points = Vec::new();
        for r in 0..=rows {
            let mut row = Vec::new();
            let ry = y + (height * r as f64) / rows as f64;
            for c in 0..=cols {
                let rx = x + (width * c as f64) / cols as f64;
                let pos = Point2D::new(rx, ry);
                row.push(MeshPoint {
                    position: pos,
                    color: Color::white(),
                    handle_top: Point2D::new(rx, ry - height / (rows as f64 * 3.0)),
                    handle_bottom: Point2D::new(rx, ry + height / (rows as f64 * 3.0)),
                    handle_left: Point2D::new(rx - width / (cols as f64 * 3.0), ry),
                    handle_right: Point2D::new(rx + width / (cols as f64 * 3.0), ry),
                });
            }
            points.push(row);
        }
        Self { rows, cols, points }
    }

    /// 특정 격자 점의 색상 변경
    pub fn set_color(&mut self, row: usize, col: usize, color: Color) {
        if row < self.points.len() && col < self.points[row].len() {
            self.points[row][col].color = color;
        }
    }

    /// Bilinear 보간으로 색상 계산 (u, v ∈ [0, 1])
    pub fn color_at(&self, u: f32, v: f32) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let col_f = u * self.cols as f32;
        let row_f = v * self.rows as f32;

        let c0 = (col_f as usize).min(self.cols as usize - 1);
        let c1 = (c0 + 1).min(self.cols as usize);
        let r0 = (row_f as usize).min(self.rows as usize - 1);
        let r1 = (r0 + 1).min(self.rows as usize);

        let fu = col_f - c0 as f32;
        let fv = row_f - r0 as f32;

        let c00 = &self.points[r0][c0].color;
        let c10 = &self.points[r0][c1].color;
        let c01 = &self.points[r1][c0].color;
        let c11 = &self.points[r1][c1].color;

        let top = c00.lerp(c10, fu);
        let bottom = c01.lerp(c11, fu);
        top.lerp(&bottom, fv)
    }
}

/// 채우기 타입 — 단색, 그라디언트, 없음
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Fill {
    None,
    Solid(Color),
    Linear(LinearGradient),
    Radial(RadialGradient),
    Mesh(MeshGradient),
}

impl Default for Fill {
    fn default() -> Self {
        Fill::Solid(Color::white())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_gradient_interpolation() {
        let grad = LinearGradient::new(Point2D::zero(), Point2D::new(100.0, 0.0));
        let c = grad.color_at(0.5);
        // 중간은 검정과 흰색의 중간
        assert!((c.r - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_mesh_gradient() {
        let mut mesh = MeshGradient::new(0.0, 0.0, 100.0, 100.0, 2, 2);
        mesh.set_color(0, 0, Color::new(1.0, 0.0, 0.0, 1.0)); // 빨강
        mesh.set_color(0, 2, Color::new(0.0, 0.0, 1.0, 1.0)); // 파랑
        let c = mesh.color_at(0.5, 0.0);
        // 빨강과 파랑 사이 중간
        assert!(c.r > 0.0 && c.b > 0.0);
    }
}
