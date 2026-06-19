# .flw (Flwing) 바이너리 포맷 규격 문서

**버전**: 1.0  
**매직 넘버**: `FLWG`  
**인코딩**: Little-endian, bincode (Rust)

## 서론
`.flw` 포맷은 Flwing 벡터 그래픽 디자인 애플리케이션의 기본 저장 형식입니다. 고성능 로딩과 저장, 그리고 폰트 데이터 및 이미지의 임베딩을 지원하도록 설계되었습니다.

## 파일 구조

파일은 헤더와 데이터 섹션으로 구성됩니다.

| 오프셋 | 크기 | 필드명 | 설명 |
| :--- | :--- | :--- | :--- |
| 0 | 4 bytes | Magic Number | 반드시 `FLWG` (0x46, 0x4C, 0x57, 0x47) 여야 함 |
| 4 | 4 bytes | Version | 파일 형식 버전 (현재 1) |
| 8 | 8 bytes | Payload Size | 이후 따라오는 데이터 섹션의 총 크기 (bytes) |
| 16 | N bytes | Data (Payload) | bincode로 직렬화된 `Document` 객체 |

## 데이터 섹션 (Payload) 구조

Data 섹션은 Rust의 `bincode` 라이브러리를 사용하여 `Document` 구조체를 직렬화한 바이너리입니다. 주요 데이터 구조는 다음과 같습니다.

### 1. Document
- `id`: UUID (16 bytes)
- `name`: String
- `artboards`: Vec<ArtBoard>
- `embedded_fonts`: Vec<EmbeddedFont>
- `file_path`: Option<String>
- `version`: u32

### 2. ArtBoard
- `id`: UUID
- `name`: String
- `width`: f64
- `height`: f64
- `background`: Color
- `layers`: Vec<Layer>

### 3. Layer
- `id`: UUID
- `name`: String
- `objects`: Vec<Object>
- `visible`: bool
- `locked`: bool
- `opacity`: f64

### 4. Object
- `id`: UUID
- `name`: String
- `content`: ObjectContent (Enum)
    - `Shape(ShapeKind)`
    - `Text(TextData)`
    - `Image(ImageData)`
    - `Group(Vec<Uuid>)`
- `transform`: Transform2D (6 x f64 행렬)
- `fill`: Fill (Enum: None, Solid, Linear, Radial, Mesh)
- `stroke`: Option<StrokeStyle>
- `effects`: Vec<Effect>
- `opacity`: f64

### 5. EmbeddedFont
- `family_name`: String
- `style`: String
- `data`: Vec<u8> (실제 TTF/OTF 파일 데이터)

## 특징
- **폰트 강제 포함**: `EmbeddedFont` 배열을 통해 사용된 폰트를 파일 내부에 직접 포함할 수 있습니다. 이를 통해 파일 이동 시에도 디자인이 유지됩니다.
- **베지에 경로**: 모든 도형은 내부적으로 베지에 곡선(BezierPath)으로 표현되거나 변환될 수 있습니다.
- **확장성**: `ObjectContent` enum을 통해 새로운 종류의 오브젝트를 쉽게 추가할 수 있습니다.
