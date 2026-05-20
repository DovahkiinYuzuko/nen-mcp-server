# NEN (Non-English Normalization) MCP Server

マルチバイト文字（日本語など）を含むファイルの解析・読み込みを強力にサポートする MCP サーバーです。Windows 環境でのパス問題や文字化けを完全に解決します。

- [日本語](#日本語)
- [ENGLISH](#english)

---

## 日本語

### 概要
NEN MCP Server は、Windows 環境における PowerShell の文字化けや、Shift-JIS などの多様なエンコーディングが混在するプロジェクトでのファイル操作を最適化するために設計されました。
特に Windows 特有の UNC パス（`\\?\` 接頭辞）によるエラーを回避し、AI エージェントが非英語圏のコードベースを正確に理解するための「目」となります。

### 主な機能
- **Windows パス問題の解決**: `dunce` を採用し、Windows の拡張パス接頭辞に起因するファイルアクセスエラーを回避します。
- **自動エンコーディング検知**: UTF-8, Shift-JIS, EUC-JP などの文字コードを自動判別し、UTF-8 に正規化して読み込みます。
- **CLI/MCP ハイブリッドモード**: サーバーとしてだけでなく、コマンドラインツールとしても動作し、直接ファイルの検証が可能です。
- **アウトライン解析**: Tree-sitter を使用し、関数の定義などを構造的に抽出します。

### インストール方法

> [!WARNING]
> Gemini CLIは2026年6月にサービス終了予定です。後継ツールである **Antigravity CLI** への移行を強く推奨します。

#### Antigravity CLI (推奨)
```bash
agy --install-extension https://github.com/DovahkiinYuzuko/nen-mcp-server
```

#### Gemini CLI (旧環境向け・非推奨)
```bash
gemini extensions install https://github.com/DovahkiinYuzuko/nen-mcp-server
```

### CLI モードの使い方
```bash
# ファイルの安全な読み込み
./bin/nen-mcp-server.exe safe-read "path/to/ファイル.txt"

# MCPサーバーとして明示的に起動
./bin/nen-mcp-server.exe --mcp
```

---

## ENGLISH

### Overview
NEN (Non-English Normalization) MCP Server is designed to optimize file operations in environments with multi-byte characters (such as Japanese) and varied encodings. It effectively resolves Windows-specific path issues and terminal encoding corruption.

### Key Features
- **UNC Path Fix**: Avoids `\\?\` prefix issues on Windows using `dunce` for reliable file access.
- **Automatic Encoding Detection**: Detects and normalizes UTF-8, Shift-JIS, and EUC-JP into standard UTF-8.
- **Hybrid CLI/MCP Mode**: Works both as a standardized MCP server and a standalone CLI tool.
- **Structural Analysis**: Extracts code structure (functions, classes) using Tree-sitter.

### Installation

> [!WARNING]
> Gemini CLI will be deprecated in June 2026. Transitioning to the successor, **Antigravity CLI**, is strongly recommended.

#### Antigravity CLI (Recommended)
```bash
agy --install-extension https://github.com/DovahkiinYuzuko/nen-mcp-server
```

#### Gemini CLI (Legacy/Deprecated)
```bash
gemini extensions install https://github.com/DovahkiinYuzuko/nen-mcp-server
```

### CLI Usage
```bash
# Standalone execution
./bin/nen-mcp-server.exe safe-read "path/to/file.txt"

# Force MCP mode
./bin/nen-mcp-server.exe --mcp
```
