# Non-EnglishNormalization (NEN) Extension Context

## 🚨 CRITICAL: FILE READING MANDATE 🚨
This workspace/extension provides the `nen-server` MCP for safe file operations. Because this environment frequently encounters Shift-JIS or other non-UTF-8 encodings (causing mojibake), you have **strict overrides** on how to read files.

### 🚫 FORBIDDEN ACTIONS
- **DO NOT USE** the built-in `read_file` tool for reading local text/code files if there is any chance of character encoding issues. If you use `read_file` and get garbled text (mojibake), **NEVER** use it again for that file.
- **DO NOT USE** `cat`, `Get-Content`, `type`, or any shell commands to read files.

### ✅ MANDATORY TOOLS (Use `nen-server` MCP)
You **MUST** prioritize the following MCP tools provided by `nen-server` (the tool names will be prefixed with `mcp_nen-server_` or similar in your tool list):

1. **`safe-read` (Direct Replacement for `read_file`)**:
   - **When to use**: **ALWAYS** prioritize this tool when you need to read the contents of a text file, source code, or documentation.
   - **Why**: It automatically detects and converts Shift-JIS/EUC-JP/UTF-8, preventing garbled text in your context.
   - **Features**: Supports reading specific ranges or tails.

2. **`get-outline`**:
   - **When to use**: When you need to understand the structure (classes, functions, methods) of a source code file without consuming tokens to read the entire file.
   - **Why**: Parses the AST via Tree-sitter and returns a lightweight JSON map of the file's architecture.

3. **`inspect-file`**:
   - **When to use**: To get file metadata (size, line count, estimated encoding) or to quickly search for a keyword and view its surrounding context (encoding-aware `grep`).

4. **`read-hex`**:
   - **When to use**: For inspecting binary files, executables, or when `safe-read` cannot decode a file. Returns a clean hex dump.

## Execution Enforcement
Before you attempt to read ANY file, remember this extension exists. If you encounter an encoding issue or garbled text, you MUST immediately switch to `safe-read`. Do not attempt to fix encoding issues via shell commands.
