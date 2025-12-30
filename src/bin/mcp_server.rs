use image_extender::process_image;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, Write};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};

// ------------------------------------------------------------------
// 1. 데이터 구조 정의 (MCP 프로토콜)
// ------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
    id: Option<Value>,
}

// 도구 호출 시 넘어올 인자 구조체
#[derive(Deserialize, Debug)]
struct ImageProcessArgs {
    file_path: String,
    width: u32,
    height: u32,
}

// ------------------------------------------------------------------
// 2. 비즈니스 로직 (lib.rs의 함수를 호출)
// ------------------------------------------------------------------

fn run_image_utility(file_path: &str, width: u32, height: u32) -> Result<String, String> {
    // [LOG] MCP 서버에서 로그는 반드시 stderr로 출력해야 합니다 (stdout은 통신용)
    eprintln!("작업 시작: {} ({}x{})", file_path, width, height);

    // 입력 검증
    if width == 0 || height == 0 {
        return Err("가로/세로 사이즈는 양수여야 합니다.".to_string());
    }

    let path = Path::new(file_path);

    // lib.rs의 process_image 함수 호출
    match process_image(path, width, height) {
        Ok(result) => {
            let message = format!(
                "성공적으로 처리되었습니다.\n입력: {}\n출력: {}\n원본 크기: {}x{}\n최종 크기: {}x{}",
                result.input_path.display(),
                result.output_path.display(),
                result.original_size.0,
                result.original_size.1,
                result.final_size.0,
                result.final_size.1
            );
            eprintln!("{}", message);
            Ok(message)
        }
        Err(e) => Err(format!("이미지 처리 실패: {}", e)),
    }
}

// ------------------------------------------------------------------
// 3. 메인 루프 (MCP 핸들러)
// ------------------------------------------------------------------

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    // MCP 서버는 종료되지 않고 계속 입력을 대기합니다.
    while reader.read_line(&mut line).await? > 0 {
        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(_) => {
                // 파싱 실패 시 무시하거나 에러 로깅
                eprintln!("JSON 파싱 실패: {}", line);
                line.clear();
                continue;
            }
        };

        let response = handle_request(request).await;

        // 응답 전송 (JSON 직렬화 + 개행문자)
        let response_str = serde_json::to_string(&response)?;
        let mut stdout = io::stdout();
        stdout.write_all(response_str.as_bytes())?;
        stdout.write_all(b"\n")?;
        stdout.flush()?;

        line.clear();
    }

    Ok(())
}

async fn handle_request(req: JsonRpcRequest) -> JsonRpcResponse {
    match req.method.as_str() {
        // [초기화] 클라이언트(Cursor/Claude)와 연결 설정
        "initialize" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id,
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "image-extender-mcp",
                    "version": "1.0.0"
                }
            })),
            error: None,
        },

        // [초기화 완료 알림]
        "notifications/initialized" => {
            // 응답이 필요 없는 알림이지만 구조상 빈 응답 처리하거나 무시
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id, // 보통 null
                result: None,
                error: None,
            }
        }

        // [도구 목록 제공] LLM에게 이 프로그램이 할 수 있는 일을 알려줍니다.
        "tools/list" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id,
            result: Some(json!({
                "tools": [{
                    "name": "extend_image",
                    "description": "PNG 이미지에 투명한 여백을 추가하여 목표 크기로 확장합니다. 원본 이미지는 중앙에 배치됩니다.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "file_path": {
                                "type": "string",
                                "description": "처리할 PNG 이미지의 절대 경로"
                            },
                            "width": {
                                "type": "integer",
                                "description": "목표 가로 크기 (px)"
                            },
                            "height": {
                                "type": "integer",
                                "description": "목표 세로 크기 (px)"
                            }
                        },
                        "required": ["file_path", "width", "height"]
                    }
                }]
            })),
            error: None,
        },

        // [도구 실행] 실제로 LLM이 도구를 사용하려고 할 때 호출됩니다.
        "tools/call" => {
            let params = req.params.unwrap_or(Value::Null);
            let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
            let args_value = params.get("arguments").cloned().unwrap_or(json!({}));

            if name == "extend_image" {
                // 인자 파싱
                match serde_json::from_value::<ImageProcessArgs>(args_value) {
                    Ok(args) => {
                        // 실제 로직 실행
                        match run_image_utility(&args.file_path, args.width, args.height) {
                            Ok(output) => JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: req.id,
                                result: Some(json!({
                                    "content": [{
                                        "type": "text",
                                        "text": output
                                    }]
                                })),
                                error: None,
                            },
                            Err(e_msg) => JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: req.id,
                                result: None,
                                error: Some(json!({
                                    "code": -32000,
                                    "message": format!("실행 오류: {}", e_msg)
                                })),
                            },
                        }
                    }
                    Err(_) => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: None,
                        error: Some(json!({
                            "code": -32602,
                            "message": "잘못된 인자 형식입니다. (file_path: string, width: int, height: int)"
                        })),
                    },
                }
            } else {
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: None,
                    error: Some(json!({
                        "code": -32601,
                        "message": "Method not found"
                    })),
                }
            }
        }

        // 그 외 요청 (Ping 등)
        _ => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id,
            result: None,
            error: Some(json!({
                "code": -32601,
                "message": "지원하지 않는 메서드입니다."
            })),
        },
    }
}

