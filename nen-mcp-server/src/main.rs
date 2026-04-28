mod mcp;
mod encoding;
mod tools;
mod parser;

use std::io::{self, BufRead, Write};
use clap::{Parser, Subcommand};
use mcp::{JsonRpcRequest, JsonRpcResponse, InitializeResult, ServerCapabilities, ServerInfo, ToolCapabilities, ToolListResult, Tool, CallToolRequest, CallToolResult, ToolContent};
use serde_json::{to_string, Value};
use tools::safe_read::safe_read;
use tools::get_outline::get_outline;
use tools::inspect_file::inspect_file;
use tools::read_hex::read_hex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Run as an MCP server (default behavior)
    #[arg(short, long)]
    mcp: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Reads a file with automatic encoding detection.
    SafeRead {
        path: String,
        /// Optional range: start end (e.g. --range 0 100)
        #[arg(short, long, num_args = 2)]
        range: Option<Vec<usize>>,
        /// Read from the end of the file
        #[arg(short, long)]
        tail: Option<usize>,
    },
    /// Extracts a high-level outline of the file (functions, classes, etc.) using Tree-sitter.
    GetOutline {
        path: String,
    },
    /// Provides detailed file metadata and optional keyword search with context.
    InspectFile {
        path: String,
        /// Keyword search query
        #[arg(short, long)]
        search: Option<String>,
    },
    /// Reads a file and returns a traditional hex dump format.
    ReadHex {
        path: String,
        /// Starting offset
        #[arg(short, long)]
        offset: Option<u64>,
        /// Number of bytes to read
        #[arg(short, long)]
        length: Option<usize>,
    },
}

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

/// Dispatches tool calls to the appropriate implementation.
fn handle_tool_call(name: &str, arguments: Value) -> CallToolResult {
    match name {
        "safe_read" => {
            let path = arguments["path"].as_str().unwrap_or("");
            let range = arguments["range"].as_array().and_then(|a| {
                if a.len() == 2 {
                    let start = a[0].as_u64()? as usize;
                    let end = a[1].as_u64()? as usize;
                    Some([start, end])
                } else {
                    None
                }
            });
            let tail = arguments["tail"].as_u64().map(|n| n as usize);

            match safe_read(path, range, tail) {
                Ok((content, encoding)) => {
                    let text = format!("[推定エンコーディング: {}]\n{}", encoding, content);
                    CallToolResult {
                        content: vec![ToolContent::Text { text }],
                        is_error: Some(false),
                    }
                },
                Err(e) => CallToolResult {
                    // {:#} を使うことで、anyhow のコンテキスト（エラーの連鎖）を表示する
                    content: vec![ToolContent::Text { text: format!("エラーが発生しました:\n{:#}", e) }],
                    is_error: Some(true),
                }
            }
        },
        "get_outline" => {
            let path = arguments["path"].as_str().unwrap_or("");
            match get_outline(path) {
                Ok(outline) => CallToolResult {
                    content: vec![ToolContent::Text { text: outline }],
                    is_error: Some(false),
                },
                Err(e) => CallToolResult {
                    content: vec![ToolContent::Text { text: format!("アウトライン取得中にエラーが発生しました:\n{:#}", e) }],
                    is_error: Some(true),
                }
            }
        },
        "inspect_file" => {
            let path = arguments["path"].as_str().unwrap_or("");
            let search_query = arguments["search_query"].as_str().map(|s| s.to_string());
            match inspect_file(path, search_query) {
                Ok(result_json) => CallToolResult {
                    content: vec![ToolContent::Text { text: result_json }],
                    is_error: Some(false),
                },
                Err(e) => CallToolResult {
                    content: vec![ToolContent::Text { text: format!("ファイル解析中にエラーが発生しました:\n{:#}", e) }],
                    is_error: Some(true),
                }
            }
        },
        "read_hex" => {
            let path = arguments["path"].as_str().unwrap_or("");
            let offset = arguments["offset"].as_u64();
            let length = arguments["length"].as_u64().map(|n| n as usize);
            match read_hex(path, offset, length) {
                Ok(hex_dump) => CallToolResult {
                    content: vec![ToolContent::Text { text: hex_dump }],
                    is_error: Some(false),
                },
                Err(e) => CallToolResult {
                    content: vec![ToolContent::Text { text: format!("バイナリ読み取り中にエラーが発生しました:\n{:#}", e) }],
                    is_error: Some(true),
                }
            }
        },
        _ => CallToolResult {
            content: vec![ToolContent::Text { text: format!("ツール '{}' は見つかりませんでした。", name) }],
            is_error: Some(true),
        }
    }
}

fn run_mcp_server() {
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
            Err(_) => continue,
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
                    version: env!("CARGO_PKG_VERSION").to_string(),
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

            let result = handle_tool_call(&call_req.name, call_req.arguments.unwrap_or(Value::Null));
            let response = JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap());
            if let Ok(res_str) = to_string(&response) {
                write_response(&res_str);
            }
        }
    }
}

fn main() {
    fix_windows_console();
    let cli = Cli::parse();

    if cli.mcp || cli.command.is_none() {
        run_mcp_server();
    } else if let Some(command) = cli.command {
        let result = match command {
            Commands::SafeRead { path, range, tail } => {
                let range_val = range.map(|r| serde_json::json!(r)).unwrap_or(Value::Null);
                let tail_val = tail.map(|t| serde_json::json!(t)).unwrap_or(Value::Null);
                let args = serde_json::json!({
                    "path": path,
                    "range": range_val,
                    "tail": tail_val
                });
                handle_tool_call("safe_read", args)
            },
            Commands::GetOutline { path } => {
                let args = serde_json::json!({ "path": path });
                handle_tool_call("get_outline", args)
            },
            Commands::InspectFile { path, search } => {
                let args = serde_json::json!({
                    "path": path,
                    "search_query": search
                });
                handle_tool_call("inspect_file", args)
            },
            Commands::ReadHex { path, offset, length } => {
                let args = serde_json::json!({
                    "path": path,
                    "offset": offset,
                    "length": length
                });
                handle_tool_call("read_hex", args)
            }
        };

        for content in result.content {
            match content {
                ToolContent::Text { text } => {
                    // Use write_all to ensure UTF-8 output without Rust's auto-conversion
                    let _ = std::io::stdout().write_all(text.as_bytes());
                    let _ = std::io::stdout().write_all(b"\n");
                }
            }
        }
        if result.is_error == Some(true) {
            std::process::exit(1);
        }
    }
}
