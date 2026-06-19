use sys_locale;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Korean,
    English,
}

static CURRENT_LANG: Lazy<RwLock<Language>> = Lazy::new(|| {
    let lang = match sys_locale::get_locale() {
        Some(l) if l.starts_with("ko") => Language::Korean,
        _ => Language::English,
    };
    RwLock::new(lang)
});

static STRINGS: Lazy<HashMap<Language, HashMap<&'static str, &'static str>>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Korean
    let mut ko = HashMap::new();
    ko.insert("app_title", "Flwing — 자유롭게 날개를 펼쳐라");
    ko.insert("file", "파일");
    ko.insert("edit", "편집");
    ko.insert("view", "보기");
    ko.insert("help", "도움말");
    ko.insert("new", "새로 만들기");
    ko.insert("open", "열기");
    ko.insert("save", "저장");
    ko.insert("save_as", "다른 이름으로 저장");
    ko.insert("exit", "종료");
    ko.insert("tools", "도구");
    ko.insert("properties", "속성");
    ko.insert("layers", "레이어");
    ko.insert("select", "선택");
    ko.insert("pen", "펜");
    ko.insert("text", "텍스트");
    ko.insert("rect", "사각형");
    ko.insert("ellipse", "타원");
    ko.insert("image_trace", "이미지 트레이스");
    ko.insert("color", "색상");
    ko.insert("fill", "채우기");
    ko.insert("stroke", "선");
    ko.insert("width", "두께");
    ko.insert("opacity", "불투명도");
    ko.insert("outline_text", "텍스트 윤곽선 만들기");
    ko.insert("font", "글꼴");
    ko.insert("size", "크기");
    ko.insert("tracking", "자간");
    ko.insert("kerning", "커닝");
    m.insert(Language::Korean, ko);

    // English
    let mut en = HashMap::new();
    en.insert("app_title", "Flwing — Spread your wings freely");
    en.insert("file", "File");
    en.insert("edit", "Edit");
    en.insert("view", "View");
    en.insert("help", "Help");
    en.insert("new", "New");
    en.insert("open", "Open");
    en.insert("save", "Save");
    en.insert("save_as", "Save As");
    en.insert("exit", "Exit");
    en.insert("tools", "Tools");
    en.insert("properties", "Properties");
    en.insert("layers", "Layers");
    en.insert("select", "Select");
    en.insert("pen", "Pen");
    en.insert("text", "Text");
    en.insert("rect", "Rectangle");
    en.insert("ellipse", "Ellipse");
    en.insert("image_trace", "Image Trace");
    en.insert("color", "Color");
    en.insert("fill", "Fill");
    en.insert("stroke", "Stroke");
    en.insert("width", "Width");
    en.insert("opacity", "Opacity");
    en.insert("outline_text", "Create Outlines");
    en.insert("font", "Font");
    en.insert("size", "Size");
    en.insert("tracking", "Tracking");
    en.insert("kerning", "Kerning");
    m.insert(Language::English, en);

    m
});

pub fn get_string(key: &str) -> String {
    let lang = *CURRENT_LANG.read().unwrap();
    STRINGS.get(&lang)
        .and_then(|m: &HashMap<&'static str, &'static str>| m.get(key))
        .map(|s: &&'static str| s.to_string())
        .unwrap_or_else(|| key.to_string())
}

pub fn set_language(lang: Language) {
    let mut l = CURRENT_LANG.write().unwrap();
    *l = lang;
}

pub fn get_current_language() -> Language {
    *CURRENT_LANG.read().unwrap()
}
