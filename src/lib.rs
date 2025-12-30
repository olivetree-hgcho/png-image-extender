//! PNG 이미지에 투명한 여백을 추가하여 목표 크기로 확장하는 라이브러리

use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 이미지 처리 결과
pub struct ProcessResult {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub original_size: (u32, u32),
    pub final_size: (u32, u32),
}

/// 디렉토리에서 모든 PNG 파일을 찾아 반환
pub fn find_png_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext.to_ascii_lowercase() == "png")
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// 단일 이미지에 여백을 추가하여 목표 크기로 확장
///
/// # Arguments
/// * `input_path` - 입력 이미지 경로
/// * `target_width` - 목표 가로 픽셀수
/// * `target_height` - 목표 세로 픽셀수
///
/// # Returns
/// 처리 결과를 담은 `ProcessResult` 또는 에러
pub fn process_image(
    input_path: &Path,
    target_width: u32,
    target_height: u32,
) -> Result<ProcessResult, Box<dyn std::error::Error>> {
    // 이미지 로드
    let img = image::open(input_path)?;
    let (current_width, current_height) = img.dimensions();

    // 최종 크기 결정 (현재 크기가 목표보다 크면 유지)
    let final_width = if current_width > target_width {
        current_width
    } else {
        target_width
    };

    let final_height = if current_height > target_height {
        current_height
    } else {
        target_height
    };

    // 투명한 캔버스 생성 (RGBA)
    let mut canvas: RgbaImage = ImageBuffer::from_pixel(
        final_width,
        final_height,
        Rgba([0, 0, 0, 0]), // 완전 투명
    );

    // 이미지를 중앙에 배치할 위치 계산
    let left_padding = (final_width - current_width) / 2;
    let top_padding = (final_height - current_height) / 2;

    // 원본 이미지를 캔버스에 복사
    let rgba_img = img.to_rgba8();
    for (x, y, pixel) in rgba_img.enumerate_pixels() {
        let new_x = x + left_padding;
        let new_y = y + top_padding;
        canvas.put_pixel(new_x, new_y, *pixel);
    }

    // 출력 경로 생성 (원본 파일이 있는 디렉토리에 ImageExtended 폴더)
    let parent_dir = input_path.parent().unwrap_or(Path::new("."));
    let output_dir = parent_dir.join("ImageExtended");
    std::fs::create_dir_all(&output_dir)?;

    let file_name = input_path.file_name().unwrap();
    let output_path = output_dir.join(file_name);

    // PNG로 저장 (명시적으로 PNG 포맷 지정)
    canvas.save_with_format(&output_path, image::ImageFormat::Png)?;

    Ok(ProcessResult {
        input_path: input_path.to_path_buf(),
        output_path,
        original_size: (current_width, current_height),
        final_size: (final_width, final_height),
    })
}

