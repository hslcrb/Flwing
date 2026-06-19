use serde::{Deserialize, Serialize};

/// 노이즈 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoiseType {
    Perlin,
    Simplex,
    Worley,
}

/// 노이즈 효과 파라미터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseEffect {
    pub noise_type: NoiseType,
    /// 노이즈 크기/주파수 (작을수록 디테일)
    pub scale: f64,
    /// 옥타브 수 (디테일 레벨)
    pub octaves: u32,
    /// 강도 (0.0 ~ 1.0)
    pub intensity: f64,
    /// 시드
    pub seed: u32,
    /// 단색 vs 색상 노이즈
    pub monochrome: bool,
}

impl Default for NoiseEffect {
    fn default() -> Self {
        Self {
            noise_type: NoiseType::Perlin,
            scale: 50.0,
            octaves: 4,
            intensity: 0.5,
            seed: 42,
            monochrome: true,
        }
    }
}

/// 텍스쳐 패턴 종류
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TexturePattern {
    /// 점 패턴
    Dots {
        spacing: f64,
        dot_radius: f64,
    },
    /// 줄무늬
    Stripes {
        spacing: f64,
        thickness: f64,
        angle: f64,
    },
    /// 격자
    Grid {
        spacing_x: f64,
        spacing_y: f64,
        line_width: f64,
    },
    /// 크로스해치
    Crosshatch {
        spacing: f64,
        angle1: f64,
        angle2: f64,
        line_width: f64,
    },
}

/// 텍스쳐 효과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureEffect {
    pub pattern: TexturePattern,
    /// 투명도 (0.0 ~ 1.0)
    pub opacity: f64,
    /// 스케일 배율
    pub scale: f64,
}

impl Default for TextureEffect {
    fn default() -> Self {
        Self {
            pattern: TexturePattern::Dots {
                spacing: 10.0,
                dot_radius: 2.0,
            },
            opacity: 0.5,
            scale: 1.0,
        }
    }
}

/// 오브젝트에 적용되는 효과 목록
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    Noise(NoiseEffect),
    Texture(TextureEffect),
    /// 불투명도
    Opacity(f64),
    /// 블러
    GaussianBlur { radius: f64 },
    /// 드롭 쉐도우
    DropShadow {
        offset_x: f64,
        offset_y: f64,
        blur_radius: f64,
        color: crate::core::types::Color,
    },
}
