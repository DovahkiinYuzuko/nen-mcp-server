mod mcp;
mod encoding;
mod tools;
mod parser;

use std::io::{self, BufRead, Write};
use mcp::{JsonRpcRequest, JsonRpcResponse, InitializeResult, ServerCapabilities, ServerInfo, ToolCapabilities, ToolListResult, Tool, CallToolRequest, CallToolResult, ToolContent};
use serde_json::to_string;
use tools::safe_read::safe_read;
use tools::get_outline::get_outline;
use tools::inspect_file::inspect_file;
use tools::read_hex::read_hex;

#[cfg(windows)]
fn fix_windows_console() {
    use windows_sys::Win32::System::Console::{SetConsoleOutputCP, SetConsoleCP};
    unsafe {
        SetConsoleOutputCP(65001);
        SetConsoleCP(65001);
    };
}

#[cfg(not(windows))]
fn fix_windows_console() {}

fn write_response(res_str: &str) {
    let mut stdout = std::io::stdout();
    stdout.write_all(res_str.as_bytes()).unwrap();
    stdout.write_all(b"\n").unwrap();
    stdout.flush().unwrap();
}

fn main() {
    fix_windows_console();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };

        if line.trim().is_empty() {
            continue;
        }

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(_) => {
                continue;
            }
        };

        if request.method == "initialize" {
            let result = InitializeResult {
                protocol_version: "2024-11-05".to_string(),
                capabilities: ServerCapabilities {
                    tools: Some(ToolCapabilities {
                        list_changed: Some(false),
                    }),
                },
                server_info: ServerInfo {
                    name: "nen-mcp-server".to_string(),
                    version: "0.1.0".to_string(),
                },
            };

            let response = JsonRpcResponse::success(
                request.id,
                serde_json::to_value(result).unwrap(),
            );

            if let Ok(res_str) = to_string(&response) {
                write_response(&res_str);
            }
        } else if request.method == "tools/list" {
            let result = ToolListResult {
                tools: vec![
                    Tool {
                        name: "safe_read".to_string(),
                        description: Some("Reads a file with automatic encoding detection.".to_string()),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "path": { "type": "string" },
                                "range": {
                                    "type": "array",
                                    "items": { "type": "integer" },
                                    "minItems": 2,
                                    "maxItems": 2
                                },
                                "tail": { "type": "integer" }
                            },
                            "required": ["path"]
                        }),
                    },
                    Tool {
                        name: "get_outline".to_string(),
                        description: Some("Extracts a high-level outline of the file (functions, classes, etc.) using Tree-sitter.".to_string()),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "path": { "type": "string" }
                            },
                            "required": ["path"]
                        }),
                    },
                    Tool {
                        name: "inspect_file".to_string(),
                        description: Some("Provides detailed file metadata and optional keyword search with context.".to_string()),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "path": { "type": "string" },
                                "search_query": { "type": "string" }
                            },
                            "required": ["path"]
                        }),
                    },
                    Tool {
                        name: "read_hex".to_string(),
                        description: Some("Reads a file and returns a traditional hex dump format.".to_string()),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "path": { "type": "string" },
                                "offset": { "type": "integer" },
                                "length": { "type": "integer" }
                            },
                            "required": ["path"]
                        }),
                    }
                ],
            };
            let response = JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap());
            if let Ok(res_str) = to_string(&response) {
                write_response(&res_str);
            }
        } else if request.method == "tools/call" || request.method == "mcp_tools/call" {
            let call_req: CallToolRequest = match serde_json::from_value(request.params.clone()) {
                Ok(req) => req,
                Err(_) => {
                    let response = JsonRpcResponse::error(request.id, -32602, "Invalid params");
                    if let Ok(res_str) = to_string(&response) {
                        write_response(&res_str);
                    }
                    continue;
                }
            };

            if call_req.name == "safe_read" {
                let args = call_req.arguments.unwrap_or(serde_json::json!({}));
                let path = args["path"].as_str().unwrap_or("");
                let range = args["range"].as_array().and_then(|a| {
                    if a.len() == 2 {
                        Some([a[0].as_u64()? as usize, a[1].as_u64()? as usize])
                    } else {
                        None
                    }
                });
                let tail = args["tail"].as_u64().map(|n| n as usize);

                match safe_read(path, range, tail) {
                    Ok((content, encoding)) => {
                        let text = format!("[Encoding: {}]\n{}", encoding, content);
                        let result = CallToolResult {
                            content: vec![ToolContent::Text { text }],
                            is_error: Some(false),
                        };
                        let response = JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap());
                        if let Ok(res_str) = to_string(&response) {
                            write_response(&res_str);
                        }
                    },
                    Err(e) => {
                        let result = CallToolResult {
                            content: vec![ToolContent::Text { text: e.to_string() }],
                            is_error: Some(true),
                        };
                        let response = JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap());
                        if let Ok(res_str) = to_string(&response) {
                            write_response(&res_str);
                        }
                    }
                }
            } else if call_req.name == "get_outline" {
                let args = call_req.arguments.unwrap_or(serde_json::json!({}));
                let path = args["path"].as_str().unwrap_or("");

                match get_outline(path) {
                    Ok(outline) => {
                        let result = CallToolResult {
                            content: vec![ToolContent::Text { text: outline }],
                            is_error: Some(false),
                        };
                        let response = JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap());
                        if let Ok(res_str) = to_string(&response) {
                            write_response(&res_str);
                        }
                    },
                    Err(e) => {
                        let result = CallToolResult {
                            content: vec![ToolContent::Text { text: e.to_string() }],
                            is_error: Some(true),
                        };
                        let response = JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap());
                        if let Ok(res_str) = to_string(&response) {
                            write_response(&res_str);
                        }
                    }
                }
            } else if call_req.name == "inspect_file" {
                let args = call_req.arguments.unwrap_or(serde_json::json!({}));
                let path = args["path"].as_str().unwrap_or("");
                let search_query = args["search_query"].as_str().map(|s| s.to_string());

                match inspect_file(path, search_query) {
                    Ok(result_json) => {
                        let result = CallToolResult {
                            content: vec![ToolContent::Text { text: result_json }],
                            is_error: Some(false),
                        };
                        let response = JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap());
                        if let Ok(res_str) = to_string(&response) {
                            write_response(&res_str);
                        }
                    },
                    Err(e) => {
                        let result = CallToolResult {
                            content: vec![ToolContent::Text { text: e.to_string() }],
                            is_error: Some(true),
                        };
                        let response = JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap());
                        if let Ok(res_str) = to_string(&response) {
                            write_response(&res_str);
                        }
                    }
                }
            } else if call_req.name == "read_hex" {
                let args = call_req.arguments.unwrap_or(serde_json::json!({}));
                let path = args["path"].as_str().unwrap_or("");
                let offset = args["offset"].as_u64();
                let length = args["length"].as_u64().map(|n| n as usize);

                match read_hex(path, offset, length) {
                    Ok(hex_dump) => {
                        let result = CallToolResult {
                            content: vec![ToolContent::Text { text: hex_dump }],
                            is_error: Some(false),
                        };
                        let response = JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap());
                        if let Ok(res_str) = to_string(&response) {
                            write_response(&res_str);
                        }
                    },
                    Err(e) => {
                        let result = CallToolResult {
                            content: vec![ToolContent::Text { text: e.to_string() }],
                            is_error: Some(true),
                        };
                        let response = JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap());
                        if let Ok(res_str) = to_string(&response) {
                            write_response(&res_str);
                        }
                    }
                }
            } else {
                let response = JsonRpcResponse::error(request.id, -32601, "Tool not found");
                if let Ok(res_str) = to_string(&response) {
                    write_response(&res_str);
                }
            }
        }
    }
}
