# Non-EnglishNormalization (NEN) Extension Context

## Overview
This extension provides the `nen-server` MCP, which is designed to handle file reading and structural parsing in environments where character encoding issues (e.g., Shift-JIS on Windows) frequently occur.

## Usage Guidelines
When assisting the user with reading local files, analyzing source code structure, or investigating binary files in this workspace, you MUST prioritize the tools provided by `nen-server` over standard shell commands (`cat`, `type`, etc.) or generic file reading tools.

### Available Tools:
1. **`safe_read`**:
   - **When to use**: Whenever you need to read the contents of a text file, source code, or documentation.
   - **Why**: It automatically detects the character encoding (e.g., Shift-JIS, UTF-8) and normalizes the output, preventing garbled text (mojibake) in your context window.
   - **Note**: Supports `range` (line/byte specification) and `tail` for reading large files efficiently.

2. **`get_outline`**:
   - **When to use**: When you need to understand the architecture, classes, or functions within a source code file without reading its entire content.
   - **Why**: Uses Tree-sitter to parse the AST and return a concise JSON structure of the file, saving tokens and improving comprehension.

3. **`inspect_file`**:
   - **When to use**: When you need metadata (size, lines, estimated encoding) or need to search for a specific keyword within a file and view its surrounding context.

4. **`read_hex`**:
   - **When to use**: When you need to inspect binary files, executables, or when `safe_read` fails to decode a file properly. Returns a traditional hex dump.

## Strict Rules
- Do NOT use `cat` or `Get-Content` to read files if `safe_read` is applicable.
- If a file appears to contain garbled text when read with a standard tool, immediately switch to using `safe_read`.
