# Image Extender

PNG 이미지에 투명한 여백을 추가하여 목표 크기로 확장하는 Rust 도구입니다.

**CLI 도구**와 **MCP 서버** 두 가지 방식으로 사용할 수 있습니다.

## 기능

- PNG 이미지를 읽어서 지정된 목표 크기로 투명한 여백을 추가
- 원본 이미지를 중앙에 배치하여 좌우/상하에 균등한 여백 추가
- 단일 파일 또는 디렉토리 내 모든 PNG 파일 일괄 처리
- 원본 이미지가 목표 크기보다 큰 경우 해당 축은 원본 크기 유지
- MCP(Model Context Protocol) 서버로 AI Agent와 연동 가능

## 프로젝트 구조

```
png-image-extender/
├── Cargo.toml              # 프로젝트 설정 및 의존성
├── Cargo.lock              # 의존성 잠금 파일
├── README.md               # 사용 설명서
├── src/
│   ├── lib.rs              # 핵심 이미지 처리 로직 (라이브러리)
│   └── bin/
│       ├── cli.rs          # CLI 바이너리
│       └── mcp_server.rs   # MCP 서버 바이너리
└── target/
    └── release/
        ├── image_extender.exe      # CLI 실행 파일
        └── image_extender_mcp.exe  # MCP 서버 실행 파일
```

## 빌드

```bash
# 모든 바이너리 빌드 (릴리즈 모드)
cargo build --release

# CLI만 빌드
cargo build --release --bin image_extender

# MCP 서버만 빌드
cargo build --release --bin image_extender_mcp
```

---

## CLI 사용법

### 명령어 형식

```bash
image_extender <PATH> <WIDTH> <HEIGHT>
```

### 인자

| 인자 | 설명 |
|------|------|
| `PATH` | PNG 파일 경로 또는 PNG 파일들이 있는 디렉토리 경로 |
| `WIDTH` | 목표 가로 픽셀수 |
| `HEIGHT` | 목표 세로 픽셀수 |

### 예시

단일 파일 처리:
```bash
image_extender image.png 512 512
```

디렉토리 내 모든 PNG 파일 처리:
```bash
image_extender ./images 1024 768
```

### 출력

처리된 이미지는 원본 파일이 위치한 디렉토리에 `ImageExtended` 폴더가 생성되고, 그 안에 동일한 파일명으로 저장됩니다.

```
원본 구조:
images/
├── a.png
└── b.png

처리 후:
images/
├── a.png
├── b.png
└── ImageExtended/
    ├── a.png
    └── b.png
```

---

## MCP 서버 사용법

MCP(Model Context Protocol) 서버를 통해 AI Agent(Claude, Cursor 등)가 이미지 확장 기능을 직접 호출할 수 있습니다.

### MCP 서버 설정

#### Cursor IDE 설정

`~/.cursor/mcp.json` 파일에 다음 내용을 추가합니다:

```json
{
  "mcpServers": {
    "image-extender": {
      "command": "C:\\Workspace\\png-image-extender\\target\\release\\image_extender_mcp.exe",
      "args": []
    }
  }
}
```

#### Claude Desktop 설정

`claude_desktop_config.json` 파일에 다음 내용을 추가합니다:

```json
{
  "mcpServers": {
    "image-extender": {
      "command": "/path/to/image_extender_mcp",
      "args": []
    }
  }
}
```

### 제공되는 MCP 도구

| 도구 이름 | 설명 |
|-----------|------|
| `extend_image` | PNG 이미지에 투명한 여백을 추가하여 목표 크기로 확장 |

### 도구 파라미터

| 파라미터 | 타입 | 설명 |
|----------|------|------|
| `file_path` | string | 처리할 PNG 이미지의 절대 경로 |
| `width` | integer | 목표 가로 크기 (px) |
| `height` | integer | 목표 세로 크기 (px) |

### MCP 사용 예시

AI Agent에게 다음과 같이 요청할 수 있습니다:

> "C:\images\logo.png 파일을 512x512 크기로 확장해줘"

AI Agent가 `extend_image` 도구를 호출하면:

```json
{
  "name": "extend_image",
  "arguments": {
    "file_path": "C:\\images\\logo.png",
    "width": 512,
    "height": 512
  }
}
```

결과로 `C:\images\ImageExtended\logo.png` 파일이 생성됩니다.

---

## 동작 방식

1. PNG 이미지를 로드합니다.
2. 가로 픽셀수가 목표보다 작으면 좌우에 투명 여백을 추가하여 중앙 정렬합니다.
3. 세로 픽셀수가 목표보다 작으면 상하에 투명 여백을 추가하여 중앙 정렬합니다.
4. 원본 크기가 목표보다 큰 경우 해당 축은 원본 크기를 유지합니다.
5. 결과를 `ImageExtended` 폴더에 저장합니다.

## 라이브러리로 사용

`image_extender`를 Rust 프로젝트에서 라이브러리로 사용할 수도 있습니다:

```rust
use image_extender::{process_image, find_png_files};
use std::path::Path;

fn main() {
    // 단일 이미지 처리
    let result = process_image(Path::new("image.png"), 512, 512).unwrap();
    println!("저장됨: {}", result.output_path.display());

    // 디렉토리에서 PNG 파일 찾기
    let png_files = find_png_files(Path::new("./images"));
    for file in png_files {
        process_image(&file, 512, 512).unwrap();
    }
}
```

## 라이선스

MIT

## 참고

Windows에서 빌드 시 GNU 툴체인 대신 MSVC 툴체인을 사용하려면:

```bash
# 일회성 빌드
cargo +stable-x86_64-pc-windows-msvc build --release

# 또는 기본 툴체인 변경
rustup default stable-x86_64-pc-windows-msvc
```