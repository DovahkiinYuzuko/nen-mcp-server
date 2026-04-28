# Non-EnglishNormalization (NEN) MCP Server

Windows環境における文字化けを解消し、Tree-sitterによる構造抽出機能を提供する高精度なMCPサーバーです。

- [日本語](#日本語)
- [English](#english)

---

## 日本語

### 概要
NEN MCP Serverは、Windows環境で発生しやすいShift-JIS等の文字コードによる文字化け問題を解決し、ソースコードや文書の構造を正確に把握するためのツール群を提供します。
`chardetng`による高精度なエンコーディング自動検出と、`tree-sitter`によるシンタックス解析を組み合わせることで、AIエージェントの理解力を向上させます。

### 提供ツール

#### 1. `safe_read`
ファイルをバイナリで読み込み、エンコーディングを自動判別してUTF-8としてデコードします。
- `path` (必須): ファイルパス
- `range` (任意): 読み込み開始・終了バイト位置 `[start, end]`
- `tail` (任意): 末尾から読み込むバイト数

#### 2. `get_outline`
Tree-sitterを使用して、ファイル内の関数やクラスの定義を構造的に抽出します。
- `path` (必須): ファイルパス
- 対応言語: Rust (`.rs`), Python (`.py`), C# (`.cs`)

#### 3. `inspect_file`
ファイルのメタデータ（サイズ、作成日時、更新日時）の取得と、キーワード検索を同時に行います。
- `path` (必須): ファイルパス
- `search_query` (任意): 検索キーワード。一致した行とその前後2行をコンテキストとして返します。

#### 4. `read_hex`
ファイルをヘキサダンプ形式で表示します。文字コード判別が困難なバイナリファイルや、破損したファイルの調査に最適です。
- `path` (必須): ファイルパス
- `offset` (任意): 開始オフセット
- `length` (任意): 読み込みバイト数（デフォルト 256）

### セットアップ

#### ビルド
```powershell
cd nen-mcp-server
cargo build --release
```

#### 設定
Claude DesktopなどのMCPクライアントで本サーバーを使用するには、設定ファイル（例: `%APPDATA%\Claude\claude_desktop_config.json`）に以下のように記述します。

```json
{
  "mcpServers": {
    "nen-server": {
      "command": "C:\\path\\to\\your\\nen-mcp-server\\target\\release\\nen-mcp-server.exe",
      "args": []
    }
  }
}
```

### 開発ルール
- すべての機能はUTF-8で正規化。
- パス検証ロジックを共通化し、Windows特有のUNCパス問題にも対応。
- Tree-sitterによる多言語サポート。

---

## English

### Overview
NEN (Non-English Normalization) MCP Server is designed to resolve encoding issues (such as Shift-JIS corruption) in Windows environments and provide structured code analysis using Tree-sitter.
By combining high-precision encoding detection via `chardetng` with syntax parsing via `tree-sitter`, it enhances the analytical capabilities of AI agents.

### Provided Tools

#### 1. `safe_read`
Reads a file in binary mode, automatically detects the encoding, and decodes it as UTF-8.
- `path` (required): File path
- `range` (optional): Start and end byte positions `[start, end]`
- `tail` (optional): Number of bytes to read from the end

#### 2. `get_outline`
Uses Tree-sitter to extract function and class definitions from a file.
- `path` (required): File path
- Supported Languages: Rust (`.rs`), Python (`.py`), C# (`.cs`)

#### 3. `inspect_file`
Retrieves file metadata (size, creation time, modification time) and performs keyword search simultaneously.
- `path` (required): File path
- `search_query` (optional): Search keyword. Returns the matching line and its surrounding context.

#### 4. `read_hex`
Displays file content in a hex dump format. Ideal for inspecting binary files or corrupted documents.
- `path` (required): File path
- `offset` (optional): Starting offset
- `length` (optional): Number of bytes to read (default: 256)

### Setup

#### Build
```powershell
cd nen-mcp-server
cargo build --release
```

#### Configuration
To use this server with an MCP client like Claude Desktop, add the following to your config file:

```json
{
  "mcpServers": {
    "nen-server": {
      "command": "C:\\path\\to\\your\\nen-mcp-server\\target\\release\\nen-mcp-server.exe",
      "args": []
    }
  }
}
```

### Development Guidelines
- All outputs are normalized to UTF-8.
- Unified path validation logic, compatible with Windows UNC paths.
- Multi-language support via Tree-sitter.
