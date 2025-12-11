# Image Extender

PNG 이미지에 투명한 여백을 추가하여 목표 크기로 확장하는 Rust CLI 도구입니다.

## 기능

- PNG 이미지를 읽어서 지정된 목표 크기로 투명한 여백을 추가
- 원본 이미지를 중앙에 배치하여 좌우/상하에 균등한 여백 추가
- 단일 파일 또는 디렉토리 내 모든 PNG 파일 일괄 처리
- 원본 이미지가 목표 크기보다 큰 경우 해당 축은 원본 크기 유지

## 빌드

```bash
cargo build --release
```

## 사용법

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

## 출력

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

## 동작 방식

1. PNG 이미지를 로드합니다.
2. 가로 픽셀수가 목표보다 작으면 좌우에 투명 여백을 추가하여 중앙 정렬합니다.
3. 세로 픽셀수가 목표보다 작으면 상하에 투명 여백을 추가하여 중앙 정렬합니다.
4. 원본 크기가 목표보다 큰 경우 해당 축은 원본 크기를 유지합니다.
5. 결과를 `ImageExtended` 폴더에 저장합니다.

## 라이선스

MIT

## 완료된 프로젝트 구조

```
ImageExtender/
├── Cargo.toml        # 프로젝트 설정 및 의존성
├── Cargo.lock        # 의존성 잠금 파일
├── README.md         # 사용 설명서
├── src/
│   └── main.rs       # 메인 소스 코드
└── target/
    └── debug/
        └── image_extender.exe  # 빌드된 실행 파일
```

## 구현 요약

1. CLI 인터페이스: clap을 사용하여 3개의 인자(경로, 가로, 세로)를 파싱

2. 이미지 처리: image 크레이트로 PNG 로드/저장 및 투명 캔버스 생성

3. 디렉토리 탐색: walkdir로 디렉토리 내 모든 PNG 파일 재귀 탐색

4. 여백 추가 로직: 목표 크기보다 작을 경우에만 중앙 정렬로 투명 여백 추가

5. 출력: 원본 위치에 ImageExtended 폴더 생성 후 동일 파일명으로 저장

## 사용 예시

```
#단일 파일 처리
image_extender.exe myimage.png 512 512

# 디렉토리 내 모든 PNG 처리
image_extender.exe ./images 1024 768
```

## 참고

현재 시스템에서 GNU 툴체인이 기본 설정되어 있어서, 빌드 시 cargo +stable-x86_64-pc-windows-msvc build 명령어를 사용하거나, rustup default stable-x86_64-pc-windows-msvc 명령으로 기본 툴체인을 변경하시면 됩니다.
