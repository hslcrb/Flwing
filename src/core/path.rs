use serde::{Deserialize, Serialize};
use crate::core::types::{Point2D, BoundingBox};

/// 경로 세그먼트 — 베지에 곡선의 기본 단위
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathSegment {
    /// 시작점으로 이동
    MoveTo(Point2D),
    /// 직선
    LineTo(Point2D),
    /// 2차 베지에 곡선 (제어점 1개)
    QuadTo { ctrl: Point2D, to: Point2D },
    /// 3차 베지에 곡선 (제어점 2개)
    CubicTo { ctrl1: Point2D, ctrl2: Point2D, to: Point2D },
    /// 경로 닫기
    Close,
}

/// 베지에 경로 — 여러 세그먼트로 구성된 벡터 경로
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BezierPath {
    pub segments: Vec<PathSegment>,
    pub closed: bool,
}

impl BezierPath {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            closed: false,
        }
    }

    pub fn move_to(&mut self, p: Point2D) {
        self.segments.push(PathSegment::MoveTo(p));
    }

    pub fn line_to(&mut self, p: Point2D) {
        self.segments.push(PathSegment::LineTo(p));
    }

    pub fn quad_to(&mut self, ctrl: Point2D, to: Point2D) {
        self.segments.push(PathSegment::QuadTo { ctrl, to });
    }

    pub fn cubic_to(&mut self, ctrl1: Point2D, ctrl2: Point2D, to: Point2D) {
        self.segments.push(PathSegment::CubicTo { ctrl1, ctrl2, to });
    }

    pub fn close(&mut self) {
        self.segments.push(PathSegment::Close);
        self.closed = true;
    }

    /// lyon Path로 변환 (테셀레이션용)
    pub fn to_lyon_path(&self) -> lyon::path::Path {
        use lyon::math::point;
        let mut builder = lyon::path::Path::builder();

        for seg in &self.segments {
            match seg {
                PathSegment::MoveTo(p) => {
                    builder.begin(point(p.x as f32, p.y as f32));
                }
                PathSegment::LineTo(p) => {
                    builder.line_to(point(p.x as f32, p.y as f32));
                }
                PathSegment::QuadTo { ctrl, to } => {
                    builder.quadratic_bezier_to(
                        point(ctrl.x as f32, ctrl.y as f32),
                        point(to.x as f32, to.y as f32),
                    );
                }
                PathSegment::CubicTo { ctrl1, ctrl2, to } => {
                    builder.cubic_bezier_to(
                        point(ctrl1.x as f32, ctrl1.y as f32),
                        point(ctrl2.x as f32, ctrl2.y as f32),
                        point(to.x as f32, to.y as f32),
                    );
                }
                PathSegment::Close => {
                    builder.close();
                }
            }
        }

        // 열린 경로의 경우 end() 호출
        if !self.closed {
            builder.end(false);
        }

        builder.build()
    }

    /// 바운딩 박스 계산
    pub fn bounding_box(&self) -> Option<BoundingBox> {
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut has_points = false;

        for seg in &self.segments {
            let points: Vec<&Point2D> = match seg {
                PathSegment::MoveTo(p) | PathSegment::LineTo(p) => vec![p],
                PathSegment::QuadTo { ctrl, to } => vec![ctrl, to],
                PathSegment::CubicTo { ctrl1, ctrl2, to } => vec![ctrl1, ctrl2, to],
                PathSegment::Close => vec![],
            };

            for p in points {
                has_points = true;
                min_x = min_x.min(p.x);
                min_y = min_y.min(p.y);
                max_x = max_x.max(p.x);
                max_y = max_y.max(p.y);
            }
        }

        if has_points {
            Some(BoundingBox::new(
                Point2D::new(min_x, min_y),
                Point2D::new(max_x, max_y),
            ))
        } else {
            None
        }
    }

    /// 사각형 경로 생성
    pub fn rectangle(x: f64, y: f64, w: f64, h: f64) -> Self {
        let mut path = Self::new();
        path.move_to(Point2D::new(x, y));
        path.line_to(Point2D::new(x + w, y));
        path.line_to(Point2D::new(x + w, y + h));
        path.line_to(Point2D::new(x, y + h));
        path.close();
        path
    }

    /// 타원 경로 생성 (4개의 3차 베지에 곡선으로 근사)
    pub fn ellipse(cx: f64, cy: f64, rx: f64, ry: f64) -> Self {
        let kappa = 0.5522847498; // 4*(sqrt(2)-1)/3
        let mut path = Self::new();

        path.move_to(Point2D::new(cx + rx, cy));
        path.cubic_to(
            Point2D::new(cx + rx, cy + ry * kappa),
            Point2D::new(cx + rx * kappa, cy + ry),
            Point2D::new(cx, cy + ry),
        );
        path.cubic_to(
            Point2D::new(cx - rx * kappa, cy + ry),
            Point2D::new(cx - rx, cy + ry * kappa),
            Point2D::new(cx - rx, cy),
        );
        path.cubic_to(
            Point2D::new(cx - rx, cy - ry * kappa),
            Point2D::new(cx - rx * kappa, cy - ry),
            Point2D::new(cx, cy - ry),
        );
        path.cubic_to(
            Point2D::new(cx + rx * kappa, cy - ry),
            Point2D::new(cx + rx, cy - ry * kappa),
            Point2D::new(cx + rx, cy),
        );
        path.close();
        path
    }
}

impl Default for BezierPath {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_path() {
        let path = BezierPath::rectangle(10.0, 20.0, 100.0, 50.0);
        assert!(path.closed);
        assert_eq!(path.segments.len(), 6); // move + 3 lines + close
        let bb = path.bounding_box().unwrap();
        assert_eq!(bb.min.x, 10.0);
        assert_eq!(bb.max.x, 110.0);
    }

    #[test]
    fn test_ellipse_path() {
        let path = BezierPath::ellipse(100.0, 100.0, 50.0, 30.0);
        assert!(path.closed);
        let bb = path.bounding_box().unwrap();
        // 대략적인 바운딩박스 확인
        assert!(bb.min.x < 60.0);
        assert!(bb.max.x > 140.0);
    }
}
