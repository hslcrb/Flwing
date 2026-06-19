use std::fs::File;
use std::io::{Read, Write};
use crate::core::document::Document;
use bincode;
use log::{info, error};

/// .flw 파일 헤더 (매직 넘버)
const MAGIC: &[u8; 4] = b"FLWG";
const VERSION: u32 = 1;

/// 문서를 .flw 바이너리 파일로 저장
pub fn save_to_file(doc: &Document, path: &str) -> bincode::Result<()> {
    let mut file = File::create(path).map_err(|e| {
        error!("파일 생성 실패: {}", e);
        bincode::ErrorKind::Io(e)
    })?;

    // 1. 헤더 작성
    file.write_all(MAGIC).map_err(|e| bincode::ErrorKind::Io(e))?;
    // 2. 버전 작성
    file.write_all(&VERSION.to_le_bytes()).map_err(|e| bincode::ErrorKind::Io(e))?;

    // 3. 데이터 직렬화
    let encoded: Vec<u8> = bincode::serialize(doc)?;
    
    // 4. 데이터 크기 작성
    let size = encoded.len() as u64;
    file.write_all(&size.to_le_bytes()).map_err(|e| bincode::ErrorKind::Io(e))?;

    // 5. 실제 데이터 작성
    file.write_all(&encoded).map_err(|e| bincode::ErrorKind::Io(e))?;

    info!("문서 저장 완료: {} ({} bytes)", path, size);
    Ok(())
}

/// .flw 바이너리 파일에서 문서 로드
pub fn load_from_file(path: &str) -> Result<Document, String> {
    let mut file = File::open(path).map_err(|e| format!("파일 열기 실패: {}", e))?;

    // 1. 헤더 검증
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic).map_err(|_| "잘못된 파일 포맷 (헤더 누락)")?;
    if &magic != MAGIC {
        return Err("잘못된 파일 포맷 (매직 넘버 불일치)".to_string());
    }

    // 2. 버전 확인
    let mut ver_buf = [0u8; 4];
    file.read_exact(&mut ver_buf).map_err(|_| "버전 정보 읽기 실패")?;
    let version = u32::from_le_bytes(ver_buf);
    if version > VERSION {
        return Err(format!("지원되지 않는 최신 버전 파일입니다 (V{})", version));
    }

    // 3. 데이터 크기 읽기
    let mut size_buf = [0u8; 8];
    file.read_exact(&mut size_buf).map_err(|_| "데이터 크기 읽기 실패")?;
    let size = u64::from_le_bytes(size_buf);

    // 4. 데이터 로드
    let mut data = vec![0u8; size as usize];
    file.read_exact(&mut data).map_err(|_| "데이터 로드 중 오류 발생")?;

    // 5. 역직렬화
    let doc: Document = bincode::deserialize(&data)
        .map_err(|e| format!("데이터 파싱 실패: {}", e))?;

    info!("문서 로드 완료: {} (V{})", path, version);
    Ok(doc)
}
