use clap::Parser;
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// PNG 이미지에 투명한 여백을 추가하여 목표 크기로 확장하는 CLI 도구
#[derive(Parser, Debug)]
#[command(name = "image_extender")]
#[command(about = "PNG 이미지에 투명한 여백을 추가하여 목표 크기로 확장합니다")]
struct Args {
    /// PNG 파일 경로 또는 디렉토리 경로
    #[arg(help = "PNG 파일 또는 디렉토리 경로")]
    path: PathBuf,

    /// 목표 가로 픽셀수
    #[arg(help = "목표 가로 픽셀수")]
    width: u32,

    /// 목표 세로 픽셀수
    #[arg(help = "목표 세로 픽셀수")]
    height: u32,
}

/// 디렉토리에서 모든 PNG 파일을 찾아 반환
fn find_png_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension()
                .map(|ext| ext.to_ascii_lowercase() == "png")
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// 단일 이미지에 여백을 추가하여 목표 크기로 확장
fn process_image(
    input_path: &Path,
    target_width: u32,
    target_height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // 이미지 로드
    let img = image::open(input_path)?;
    let (current_width, current_height) = img.dimensions();

    println!(
        "처리 중: {} ({}x{} -> {}x{})",
        input_path.display(),
        current_width,
        current_height,
        target_width,
        target_height
    );

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

    println!("저장 완료: {}", output_path.display());
    Ok(())
}

fn main() {
    let args = Args::parse();

    let path = &args.path;
    let target_width = args.width;
    let target_height = args.height;

    // 경로가 파일인지 디렉토리인지 확인
    if path.is_file() {
        // 단일 파일 처리
        if let Err(e) = process_image(path, target_width, target_height) {
            eprintln!("오류: {} - {}", path.display(), e);
        }
    } else if path.is_dir() {
        // 디렉토리 내 모든 PNG 파일 처리
        let png_files = find_png_files(path);

        if png_files.is_empty() {
            println!("디렉토리에서 PNG 파일을 찾을 수 없습니다: {}", path.display());
            return;
        }

        println!("{}개의 PNG 파일을 찾았습니다.", png_files.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for file_path in &png_files {
            match process_image(file_path, target_width, target_height) {
                Ok(_) => success_count += 1,
                Err(e) => {
                    eprintln!("오류: {} - {}", file_path.display(), e);
                    error_count += 1;
                }
            }
        }

        println!(
            "\n완료: 성공 {}, 실패 {}",
            success_count, error_count
        );
    } else {
        eprintln!("유효하지 않은 경로입니다: {}", path.display());
        std::process::exit(1);
    }
}
