# NEN (Non-English Normalization) MCP Server

マルチバイト文字（日本語など）を含むファイルの解析・読み込みを強力にサポートする MCP サーバーです。

- [日本語](#日本語)
- [ENGLISH](#english)

---

## 日本語

### 概要
NEN MCP Server は、Windows 環境における PowerShell の文字化けや、Shift-JIS などの多様なエンコーディングが混在するプロジェクトでのファイル操作を最適化するために設計されました。LLM が非英語圏のコードベースを正確に理解するための「目」となります。

### 主な機能
- **自動エンコーディング検知**: UTF-8, Shift-JIS, EUC-JP などの文字コードを自動で判別し、正規化して読み込みます。
- **バイナリ・ヘキサ表示**: テキストとして読み込めないファイルの中身を 16 進数で確認できます。
- **アウトライン解析**: ソースコードの構造（クラス、関数など）を高速に抽出します。
- **安全な読み込み**: 巨大なファイルや壊れたエンコーディングのファイルでも、クラッシュせずに安全に処理します。

### インストール方法
Gemini CLI を使用している場合、以下のコマンドで簡単にインストールできます。

```bash
gemini extensions install https://github.com/DovahkiinYuzuko/nen-mcp-server
```

---

## ENGLISH

### Overview
NEN (Non-English Normalization) MCP Server is designed to optimize file operations in projects where multi-byte characters (such as Japanese) and various encodings like Shift-JIS are mixed. It acts as the "eyes" for LLMs to accurately understand non-English codebases.

### Key Features
- **Automatic Encoding Detection**: Automatically detects and normalizes character codes such as UTF-8, Shift-JIS, and EUC-JP.
- **Hex View**: Inspect the contents of files that cannot be read as text in hexadecimal format.
- **Outline Analysis**: Quickly extract the structure of source code (classes, functions, etc.).
- **Safe Read**: Safely processes large or corrupted encoding files without crashing.

### Installation
If you are using Gemini CLI, you can easily install it with the following command:

```bash
gemini extensions install https://github.com/DovahkiinYuzuko/nen-mcp-server
```

### CLI Mode
You can also run the server directly from the terminal for testing.

```bash
# Safe read
./nen-mcp-server safe-read path/to/file.txt

# Get outline
./nen-mcp-server get-outline path/to/code.rs

# Inspect file
./nen-mcp-server inspect-file path/to/file.txt --search "keyword"

# Read hex
./nen-mcp-server read-hex path/to/binary.bin
```

```bash
gemini extensions install https://github.com/DovahkiinYuzuko/nen-mcp-server
```
