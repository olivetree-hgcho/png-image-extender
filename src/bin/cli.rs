use clap::Parser;
use image_extender::{find_png_files, process_image};
use std::path::PathBuf;

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

fn main() {
    let args = Args::parse();

    let path = &args.path;
    let target_width = args.width;
    let target_height = args.height;

    // 경로가 파일인지 디렉토리인지 확인
    if path.is_file() {
        // 단일 파일 처리
        match process_image(path, target_width, target_height) {
            Ok(result) => {
                println!(
                    "처리 중: {} ({}x{} -> {}x{})",
                    result.input_path.display(),
                    result.original_size.0,
                    result.original_size.1,
                    result.final_size.0,
                    result.final_size.1
                );
                println!("저장 완료: {}", result.output_path.display());
            }
            Err(e) => {
                eprintln!("오류: {} - {}", path.display(), e);
            }
        }
    } else if path.is_dir() {
        // 디렉토리 내 모든 PNG 파일 처리
        let png_files = find_png_files(path);

        if png_files.is_empty() {
            println!(
                "디렉토리에서 PNG 파일을 찾을 수 없습니다: {}",
                path.display()
            );
            return;
        }

        println!("{}개의 PNG 파일을 찾았습니다.", png_files.len());

        let mut success_count = 0;
        let mut error_count = 0;

        for file_path in &png_files {
            match process_image(file_path, target_width, target_height) {
                Ok(result) => {
                    println!(
                        "처리 중: {} ({}x{} -> {}x{})",
                        result.input_path.display(),
                        result.original_size.0,
                        result.original_size.1,
                        result.final_size.0,
                        result.final_size.1
                    );
                    println!("저장 완료: {}", result.output_path.display());
                    success_count += 1;
                }
                Err(e) => {
                    eprintln!("오류: {} - {}", file_path.display(), e);
                    error_count += 1;
                }
            }
        }

        println!("\n완료: 성공 {}, 실패 {}", success_count, error_count);
    } else {
        eprintln!("유효하지 않은 경로입니다: {}", path.display());
        std::process::exit(1);
    }
}

