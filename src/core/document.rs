use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::types::{Color, Transform2D, StrokeStyle, FillRule};
use crate::core::shape::ShapeKind;
use crate::core::gradient::Fill;
use crate::core::effects::Effect;

/// 텍스트 오브젝트 데이터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextData {
    pub content: String,
    pub font_family: String,
    pub font_size: f64,
    /// 자간 (tracking) — em 단위
    pub tracking: f64,
    /// 커닝 활성화 여부
    pub kerning_enabled: bool,
    /// 종횡비 X 스케일 (1.0 = 기본)
    pub scale_x: f64,
    /// 종횡비 Y 스케일 (1.0 = 기본)
    pub scale_y: f64,
    /// 줄 간격 배율
    pub line_height: f64,
    /// 볼드
    pub bold: bool,
    /// 이탤릭
    pub italic: bool,
    /// 텍스트 윤곽선 변환 여부 (true면 OutlineData로 변환됨)
    pub outlined: bool,
}

impl Default for TextData {
    fn default() -> Self {
        Self {
            content: String::new(),
            font_family: "sans-serif".to_string(),
            font_size: 24.0,
            tracking: 0.0,
            kerning_enabled: true,
            scale_x: 1.0,
            scale_y: 1.0,
            line_height: 1.2,
            bold: false,
            italic: false,
            outlined: false,
        }
    }
}

/// 이미지 오브젝트 데이터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    /// 원본 파일 경로 (참조)
    pub source_path: Option<String>,
    /// 임베디드 이미지 데이터 (PNG 바이트)
    pub embedded_data: Option<Vec<u8>>,
    pub width: f64,
    pub height: f64,
}

/// 임베디드 폰트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedFont {
    pub family_name: String,
    pub style: String,
    /// 폰트 파일 바이너리 (TTF/OTF)
    pub data: Vec<u8>,
}

/// 오브젝트 콘텐츠 종류
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectContent {
    Shape(ShapeKind),
    Text(TextData),
    Image(ImageData),
    /// 그룹 (다른 오브젝트의 ID 목록)
    Group(Vec<Uuid>),
}

/// 캔버스 위의 하나의 오브젝트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub id: Uuid,
    pub name: String,
    pub content: ObjectContent,
    pub transform: Transform2D,
    pub fill: Fill,
    pub stroke: Option<StrokeStyle>,
    pub fill_rule: FillRule,
    pub effects: Vec<Effect>,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
}

impl Object {
    pub fn new_shape(name: &str, shape: ShapeKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            content: ObjectContent::Shape(shape),
            transform: Transform2D::identity(),
            fill: Fill::Solid(Color::white()),
            stroke: Some(StrokeStyle::default()),
            fill_rule: FillRule::NonZero,
            effects: Vec::new(),
            visible: true,
            locked: false,
            opacity: 1.0,
        }
    }

    pub fn new_text(name: &str, text: TextData) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            content: ObjectContent::Text(text),
            transform: Transform2D::identity(),
            fill: Fill::Solid(Color::black()),
            stroke: None,
            fill_rule: FillRule::NonZero,
            effects: Vec::new(),
            visible: true,
            locked: false,
            opacity: 1.0,
        }
    }

    pub fn new_image(name: &str, image_data: ImageData) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            content: ObjectContent::Image(image_data),
            transform: Transform2D::identity(),
            fill: Fill::None,
            stroke: None,
            fill_rule: FillRule::NonZero,
            effects: Vec::new(),
            visible: true,
            locked: false,
            opacity: 1.0,
        }
    }
}

/// 레이어
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: Uuid,
    pub name: String,
    pub objects: Vec<Object>,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
}

impl Layer {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            objects: Vec::new(),
            visible: true,
            locked: false,
            opacity: 1.0,
        }
    }

    pub fn add_object(&mut self, obj: Object) {
        self.objects.push(obj);
    }

    pub fn remove_object(&mut self, id: Uuid) -> Option<Object> {
        if let Some(pos) = self.objects.iter().position(|o| o.id == id) {
            Some(self.objects.remove(pos))
        } else {
            None
        }
    }
}

/// 아트보드
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtBoard {
    pub id: Uuid,
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub background: Color,
    pub layers: Vec<Layer>,
}

impl ArtBoard {
    pub fn new(name: &str, width: f64, height: f64) -> Self {
        let default_layer = Layer::new("Layer 1");
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            width,
            height,
            background: Color::white(),
            layers: vec![default_layer],
        }
    }

    pub fn active_layer_mut(&mut self) -> &mut Layer {
        self.layers.last_mut().expect("아트보드에 레이어가 없음")
    }

    pub fn add_layer(&mut self, name: &str) {
        self.layers.push(Layer::new(name));
    }
}

/// 문서 — 최상위 컨테이너
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub name: String,
    pub artboards: Vec<ArtBoard>,
    /// 문서에 임베디드된 폰트
    pub embedded_fonts: Vec<EmbeddedFont>,
    /// 파일 경로
    pub file_path: Option<String>,
    /// 문서 버전
    pub version: u32,
}

impl Document {
    pub fn new(name: &str) -> Self {
        let artboard = ArtBoard::new("Artboard 1", 800.0, 600.0);
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            artboards: vec![artboard],
            embedded_fonts: Vec::new(),
            file_path: None,
            version: 1,
        }
    }

    pub fn active_artboard_mut(&mut self) -> &mut ArtBoard {
        self.artboards.first_mut().expect("문서에 아트보드가 없음")
    }

    /// 폰트 임베딩 — 폰트 바이너리를 문서에 저장
    pub fn embed_font(&mut self, family: &str, style: &str, data: Vec<u8>) {
        // 중복 방지
        if self.embedded_fonts.iter().any(|f| f.family_name == family && f.style == style) {
            return;
        }
        self.embedded_fonts.push(EmbeddedFont {
            family_name: family.to_string(),
            style: style.to_string(),
            data,
        });
    }
}
