use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use log::{info, warn};

/// 로드된 폰트 정보
#[derive(Debug, Clone)]
pub struct FontInfo {
    pub family_name: String,
    pub style: String,
    pub file_path: PathBuf,
    /// 폰트 바이너리 데이터 (임베딩용)
    pub data: Vec<u8>,
}

/// 폰트 매니저 — 시스템 폰트 스캔 및 폰트 로딩/임베딩 관리
pub struct FontManager {
    /// family_name → Vec<FontInfo> 매핑
    fonts: HashMap<String, Vec<FontInfo>>,
    /// 검색할 폰트 디렉터리
    font_dirs: Vec<PathBuf>,
}

impl FontManager {
    pub fn new() -> Self {
        let font_dirs = Self::default_font_dirs();
        let mut manager = Self {
            fonts: HashMap::new(),
            font_dirs,
        };
        manager.scan_system_fonts();
        manager
    }

    /// 시스템 기본 폰트 디렉터리
    fn default_font_dirs() -> Vec<PathBuf> {
        let mut dirs = Vec::new();

        // Linux
        dirs.push(PathBuf::from("/usr/share/fonts"));
        dirs.push(PathBuf::from("/usr/local/share/fonts"));
        if let Some(home) = std::env::var_os("HOME") {
            dirs.push(PathBuf::from(home).join(".fonts"));
            dirs.push(PathBuf::from(std::env::var_os("HOME").unwrap()).join(".local/share/fonts"));
        }

        // macOS
        dirs.push(PathBuf::from("/System/Library/Fonts"));
        dirs.push(PathBuf::from("/Library/Fonts"));

        // Windows
        if let Some(windir) = std::env::var_os("WINDIR") {
            dirs.push(PathBuf::from(windir).join("Fonts"));
        }

        dirs
    }

    /// 시스템 폰트 스캔
    pub fn scan_system_fonts(&mut self) {
        for dir in self.font_dirs.clone() {
            if dir.exists() {
                self.scan_directory(&dir);
            }
        }
        info!("폰트 매니저: {} 폰트 패밀리 로드됨", self.fonts.len());
    }

    fn scan_directory(&mut self, dir: &Path) {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                self.scan_directory(&path);
            } else if Self::is_font_file(&path) {
                self.try_load_font(&path);
            }
        }
    }

    fn is_font_file(path: &Path) -> bool {
        matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("ttf" | "otf" | "ttc" | "woff" | "woff2")
        )
    }

    fn try_load_font(&mut self, path: &Path) {
        let data = match fs::read(path) {
            Ok(d) => d,
            Err(_) => return,
        };

        let face = match ttf_parser::Face::parse(&data, 0) {
            Ok(f) => f,
            Err(_) => return,
        };

        let family_name = face
            .names()
            .into_iter()
            .find(|n| n.name_id == ttf_parser::name_id::FAMILY)
            .and_then(|n| n.to_string())
            .unwrap_or_else(|| {
                path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            });

        let style = face
            .names()
            .into_iter()
            .find(|n| n.name_id == ttf_parser::name_id::SUBFAMILY)
            .and_then(|n| n.to_string())
            .unwrap_or_else(|| "Regular".to_string());

        let font_info = FontInfo {
            family_name: family_name.clone(),
            style,
            file_path: path.to_path_buf(),
            data,
        };

        self.fonts
            .entry(family_name)
            .or_insert_with(Vec::new)
            .push(font_info);
    }

    /// 폰트 패밀리 목록 반환
    pub fn font_families(&self) -> Vec<String> {
        let mut families: Vec<String> = self.fonts.keys().cloned().collect();
        families.sort();
        families
    }

    /// 특정 폰트 패밀리의 스타일 목록
    pub fn font_styles(&self, family: &str) -> Vec<String> {
        self.fonts
            .get(family)
            .map(|infos| infos.iter().map(|i| i.style.clone()).collect())
            .unwrap_or_default()
    }

    /// 폰트 데이터 가져오기 (임베딩용)
    pub fn get_font_data(&self, family: &str, style: &str) -> Option<&Vec<u8>> {
        self.fonts.get(family).and_then(|infos| {
            infos
                .iter()
                .find(|i| i.style == style)
                .or(infos.first())
                .map(|i| &i.data)
        })
    }

    /// 폰트 바이너리로부터 직접 로딩 (임베디드 폰트용)
    pub fn load_from_data(&mut self, family: &str, style: &str, data: Vec<u8>) {
        let font_info = FontInfo {
            family_name: family.to_string(),
            style: style.to_string(),
            file_path: PathBuf::new(),
            data,
        };
        self.fonts
            .entry(family.to_string())
            .or_insert_with(Vec::new)
            .push(font_info);
    }

    /// ttf-parser Face 반환
    pub fn get_face<'a>(&'a self, family: &str, style: &str) -> Option<ttf_parser::Face<'a>> {
        self.fonts.get(family).and_then(|infos| {
            let info = infos
                .iter()
                .find(|i| i.style == style)
                .or(infos.first())?;
            ttf_parser::Face::parse(&info.data, 0).ok()
        })
    }
}
