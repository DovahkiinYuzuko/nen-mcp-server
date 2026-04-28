# Non-EnglishNormalization (NEN) MCP Server

Windows環境における文字化けを解消し、Tree-sitterによる構造抽出機能を提供する高精度なMCPサーバーです。

- [日本語](#日本語)
- [English](#english)

---

## 日本語

### 概要
NEN MCP Serverは、Windows環境で発生しやすいShift-JIS等の文字コードによる文字化け問題を解決し、ソースコードや文書の構造を正確に把握するためのツール群を提供します。
`chardetng`による高精度なエンコーディング自動検出と、`tree-sitter`によるシンタックス解析を組み合わせることで、AIエージェントの理解力を向上させます。

特に、巨大なファイルのピンポイント読み込み（range/tail）や、20種類以上のプログラミング言語・マークアップ言語の構造解析に対応しています。

### 提供ツール

#### 1. `safe_read`
ファイルをバイナリで読み込み、エンコーディングを自動判別してUTF-8としてデコードします。
- `path` (必須): ファイルパス
- `range` (任意): 読み込み開始・終了バイト位置 `[start, end]`。巨大なログの特定箇所の読み込みに最適です。
- `tail` (任意): 末尾から読み込むバイト数。ログの最新状況の確認に便利です。

#### 2. `get_outline`
Tree-sitterを使用して、ファイル内の関数、クラス、メソッド、型定義などを構造的に抽出します。
- `path` (必須): ファイルパス
- **対応言語 (20種類以上)**:
  - **Programming**: Rust, Python, C#, Java, C, C++, Go, Kotlin, Ruby, Swift
  - **Web & Scripts**: JavaScript, TypeScript, PHP, **PowerShell (ps1/psm1)**
  - **Configs & Data**: JSON, TOML, YAML, SQL, Dockerfile
  - **Markup**: HTML, CSS, Markdown

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
Gemini CLIやClaude DesktopなどのMCPクライアントで本サーバーを使用するには、設定ファイルに以下のように記述します。

```json
{
  "mcpServers": {
    "nen-server": {
      "command": "C:\\path\\to\\your\\project\\bin\\nen-mcp-server.exe",
      "args": ["--mcp"]
    }
  }
}
```

### 開発・設計思想
- **UTF-8正規化**: すべてのテキスト出力はUTF-8で統一。
- **Windows完全対応**: `dunce` クレートを採用し、UNCパス (`\\?\`) に起因するエラーを回避。
- **モジュール化解析**: 言語ごとに解析ロジックを分散配置し、AIエージェントの処理負荷を軽減。

---

## English

### Overview
NEN (Non-English Normalization) MCP Server is designed to resolve encoding issues (such as Shift-JIS corruption) in Windows environments and provide structured code analysis using Tree-sitter.
By combining high-precision encoding detection via `chardetng` with syntax parsing via `tree-sitter`, it enhances the analytical capabilities of AI agents.

It supports targeted reading of large files (range/tail) and structural analysis for over 20 programming and markup languages.

### Provided Tools

#### 1. `safe_read`
Reads a file in binary mode, automatically detects the encoding, and decodes it as UTF-8.
- `path` (required): File path
- `range` (optional): Start and end byte positions `[start, end]`.
- `tail` (optional): Number of bytes to read from the end.

#### 2. `get_outline`
Uses Tree-sitter to extract function, class, and method definitions from a file.
- `path` (required): File path
- **Supported Languages**:
  - Rust, Python, C#, Java, C, C++, Go, Kotlin, Ruby, Swift, JavaScript, TypeScript, PHP, **PowerShell**, JSON, TOML, YAML, SQL, Dockerfile, HTML, CSS, Markdown.

#### 3. `inspect_file`
Retrieves file metadata (size, creation time, modification time) and performs keyword search simultaneously.
- `path` (required): File path
- `search_query` (optional): Search keyword. Returns matching lines with context.

#### 4. `read_hex`
Displays file content in a hex dump format. Ideal for inspecting binary files or corrupted documents.

### Setup & Configuration
- **Build**: `cargo build --release`
- **Execution**: Can be run as a standalone CLI or an MCP server using the `--mcp` flag.
- **Path Handling**: Compatible with Windows UNC paths via `dunce`.
