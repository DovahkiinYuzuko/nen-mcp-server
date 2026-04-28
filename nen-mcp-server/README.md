# Non-EnglishNormalization (NEN) MCP Server

Windows環境の文字化けを撲滅し、Tree-sitterで構造抽出を行う最強のMCPサーバー。

## 概要
NEN MCP Serverは、特にWindows環境における多種多様な文字コード（Shift-JISなど）による文字化け問題を解決し、ソースコードの構造を正確に把握するためのツールを提供します。
`chardetng`による高精度なエンコーディング検出と、`tree-sitter`によるシンタックス解析を組み合わせることで、開発効率を爆上げします。

## 提供ツール

### 1. `safe_read`
ファイルをバイナリで読み込み、エンコーディングを自動判別してUTF-8としてデコードします。
- `path` (required): ファイルパス
- `range` (optional): 読み込み開始・終了バイト位置 `[start, end]`
- `tail` (optional): 末尾から読み込むバイト数

### 2. `get_outline`
Tree-sitterを使用して、ファイル内の関数やクラスの定義を抽出します。
- `path` (required): ファイルパス
- 対応言語: Rust (`.rs`), Python (`.py`), C# (`.cs`)

### 3. `inspect_file`
ファイルのメタデータ（サイズ、作成日時、更新日時）の取得と、キーワード検索を同時に行います。
- `path` (required): ファイルパス
- `search_query` (optional): 検索キーワード。一致した行とその前後2行をコンテキストとして返します。

### 4. `read_hex`
ファイルをヘキサダンプ形式で表示します。文字コード判別が困難なバイナリファイルや、破損したファイルの調査に最適です。
- `path` (required): ファイルパス
- `offset` (optional): 開始オフセット
- `length` (optional): 読み込みバイト数（デフォルト 256）

## セットアップ

### ビルド
```powershell
cd nen-mcp-server
cargo build --release
```

### 設定
Claude DesktopなどのMCPクライアントで本サーバーを使用するには、設定ファイル（例: `%APPDATA%\Claude\claude_desktop_config.json`）に以下のように記述します。

`mcp_config_example.json` を参考に、パスを実際の環境に合わせて修正してください：

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

## 開発ルール
- すべての機能はUTF-8で正規化。
- パス検証ロジックを共通化し、堅牢なエラーハンドリングを実現。
- Tree-sitterによる多言語サポート。
