use serde::{Deserialize, Serialize};
use crate::core::types::Point2D;
use crate::core::path::BezierPath;

/// 기본 도형 타입
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShapeKind {
    /// 사각형
    Rectangle {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        corner_radius: f64,
    },
    /// 타원
    Ellipse {
        center: Point2D,
        radius_x: f64,
        radius_y: f64,
    },
    /// 정다각형
    Polygon {
        center: Point2D,
        radius: f64,
        sides: u32,
        rotation: f64,
    },
    /// 별
    Star {
        center: Point2D,
        outer_radius: f64,
        inner_radius: f64,
        points: u32,
        rotation: f64,
    },
    /// 자유 경로
    FreePath(BezierPath),
}

impl ShapeKind {
    /// 도형을 BezierPath로 변환
    pub fn to_path(&self) -> BezierPath {
        match self {
            ShapeKind::Rectangle { x, y, width, height, corner_radius } => {
                if *corner_radius <= 0.0 {
                    BezierPath::rectangle(*x, *y, *width, *height)
                } else {
                    Self::rounded_rect(*x, *y, *width, *height, *corner_radius)
                }
            }
            ShapeKind::Ellipse { center, radius_x, radius_y } => {
                BezierPath::ellipse(center.x, center.y, *radius_x, *radius_y)
            }
            ShapeKind::Polygon { center, radius, sides, rotation } => {
                Self::polygon_path(*center, *radius, *sides, *rotation)
            }
            ShapeKind::Star { center, outer_radius, inner_radius, points, rotation } => {
                Self::star_path(*center, *outer_radius, *inner_radius, *points, *rotation)
            }
            ShapeKind::FreePath(path) => path.clone(),
        }
    }

    fn rounded_rect(x: f64, y: f64, w: f64, h: f64, r: f64) -> BezierPath {
        let r = r.min(w / 2.0).min(h / 2.0);
        let kappa = 0.5522847498 * r;
        let mut path = BezierPath::new();

        path.move_to(Point2D::new(x + r, y));
        path.line_to(Point2D::new(x + w - r, y));
        path.cubic_to(
            Point2D::new(x + w - r + kappa, y),
            Point2D::new(x + w, y + r - kappa),
            Point2D::new(x + w, y + r),
        );
        path.line_to(Point2D::new(x + w, y + h - r));
        path.cubic_to(
            Point2D::new(x + w, y + h - r + kappa),
            Point2D::new(x + w - r + kappa, y + h),
            Point2D::new(x + w - r, y + h),
        );
        path.line_to(Point2D::new(x + r, y + h));
        path.cubic_to(
            Point2D::new(x + r - kappa, y + h),
            Point2D::new(x, y + h - r + kappa),
            Point2D::new(x, y + h - r),
        );
        path.line_to(Point2D::new(x, y + r));
        path.cubic_to(
            Point2D::new(x, y + r - kappa),
            Point2D::new(x + r - kappa, y),
            Point2D::new(x + r, y),
        );
        path.close();
        path
    }

    fn polygon_path(center: Point2D, radius: f64, sides: u32, rotation: f64) -> BezierPath {
        let mut path = BezierPath::new();
        let sides = sides.max(3);

        for i in 0..sides {
            let angle = rotation + (2.0 * std::f64::consts::PI * i as f64) / sides as f64;
            let p = Point2D::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );
            if i == 0 {
                path.move_to(p);
            } else {
                path.line_to(p);
            }
        }
        path.close();
        path
    }

    fn star_path(center: Point2D, outer_r: f64, inner_r: f64, points: u32, rotation: f64) -> BezierPath {
        let mut path = BezierPath::new();
        let points = points.max(3);
        let total = points * 2;

        for i in 0..total {
            let angle = rotation + (2.0 * std::f64::consts::PI * i as f64) / total as f64;
            let r = if i % 2 == 0 { outer_r } else { inner_r };
            let p = Point2D::new(
                center.x + r * angle.cos(),
                center.y + r * angle.sin(),
            );
            if i == 0 {
                path.move_to(p);
            } else {
                path.line_to(p);
            }
        }
        path.close();
        path
    }
}
