# rss2tg

RSSフィードを監視し、新しいエントリーをTelegramに通知するRustアプリケーション

## 機能

- RSSフィードの定期監視
- 新規エントリーの検出
- Telegram Botを通じた通知
- SQLiteによる通知履歴の管理（重複通知の防止）

## 必要要件

- Telegram Bot Token
- Telegram Chat ID

## インストール

### オプション1: ビルド済みバイナリを使用（推奨）

[Releases](https://github.com/your-username/rss2tg/releases)ページから最新のLinux用バイナリをダウンロードしてください：

```bash
# バイナリをダウンロード
wget https://github.com/your-username/rss2tg/releases/latest/download/rss2tg-linux-x86_64.tar.gz

# 解凍
tar -xzf rss2tg-linux-x86_64.tar.gz

# 実行権限を付与
chmod +x rss2tg

# 任意の場所に移動（例：/usr/local/bin）
sudo mv rss2tg /usr/local/bin/
```

### オプション2: ソースからビルド

#### 1. Rustのインストール

Rustがインストールされていない場合は、以下のコマンドでインストールしてください：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 2. プロジェクトのクローン

```bash
git clone <repository-url>
cd rss2tg
```

#### 3. ビルド

```bash
cargo build --release
```

ビルドされたバイナリは`target/release/rss2tg`に生成されます。

## セットアップ

### 1. Telegram Botの作成

1. Telegramで[@BotFather](https://t.me/botfather)を開く
2. `/newbot`コマンドを送信
3. Botの名前とユーザー名を設定
4. 発行されたBot Tokenを保存

### 2. Chat IDの取得

1. 作成したBotに何かメッセージを送信
2. ブラウザで以下のURLにアクセス：
   ```
   https://api.telegram.org/bot<YOUR_BOT_TOKEN>/getUpdates
   ```
3. レスポンスから`chat.id`の値を取得

### 3. 環境変数の設定

以下の環境変数を設定してください：

```bash
export TELEGRAM_TOKEN="your_bot_token_here"
export CHAT_ID="your_chat_id_here"
```

または、`.env`ファイルを作成して管理することもできます（推奨）。

### 4. RSS URLの設定

[`src/main.rs`](src/main.rs:14)の14行目にある`rss_url`を監視したいRSSフィードのURLに変更してください：

```rust
let rss_url = "https://example.com/rss"; // 監視対象のRSS URLに変更
```

**注意**: ソースからビルドした場合は、コードを編集してから再ビルドしてください。バイナリを使用する場合は、環境変数やコマンドライン引数でRSS URLを指定できるように改修が必要です。

## 使い方

### 手動実行

```bash
# バイナリを使用する場合
TELEGRAM_TOKEN="your_token" CHAT_ID="your_chat_id" rss2tg

# ソースから実行する場合
TELEGRAM_TOKEN="your_token" CHAT_ID="your_chat_id" cargo run --release
```

### 定期実行（cron）

定期的にRSSフィードをチェックする場合は、cronを使用します：

```bash
# crontabを編集
crontab -e

# 例：5分ごとに実行（バイナリを使用）
*/5 * * * * TELEGRAM_TOKEN="your_token" CHAT_ID="your_chat_id" /usr/local/bin/rss2tg
```

### systemdサービスとして実行

定期実行をsystemdで管理する場合：

1. サービスファイルを作成：`/etc/systemd/system/rss2tg.service`

```ini
[Unit]
Description=RSS to Telegram Notifier
After=network.target

[Service]
Type=oneshot
Environment="TELEGRAM_TOKEN=your_token"
Environment="CHAT_ID=your_chat_id"
WorkingDirectory=/var/lib/rss2tg
ExecStart=/usr/local/bin/rss2tg

[Install]
WantedBy=multi-user.target
```

2. タイマーファイルを作成：`/etc/systemd/system/rss2tg.timer`

```ini
[Unit]
Description=Run RSS to Telegram Notifier every 5 minutes

[Timer]
OnBootSec=5min
OnUnitActiveSec=5min

[Install]
WantedBy=timers.target
```

3. サービスを有効化：

```bash
sudo systemctl daemon-reload
sudo systemctl enable rss2tg.timer
sudo systemctl start rss2tg.timer
```

## データベース

アプリケーションは`rss_data.db`というSQLiteデータベースファイルを作成し、通知済みのエントリーIDを記録します。これにより、同じエントリーが複数回通知されることを防ぎます。

## トラブルシューティング

### エラー: "environment variable not found"

環境変数`TELEGRAM_TOKEN`または`CHAT_ID`が設定されていません。上記のセットアップ手順を確認してください。

### 通知が届かない

1. Bot Tokenが正しいか確認
2. Chat IDが正しいか確認
3. BotとのチャットでBotをブロックしていないか確認
4. RSS URLが正しくアクセスできるか確認

### ビルドエラー

```bash
# 依存関係を更新
cargo update

# クリーンビルド
cargo clean
cargo build --release
```

## 開発者向け情報

### 自動リリース

このプロジェクトはGitHub Actionsを使用して、**mainまたはmasterブランチへのコミット時に自動的に**Linux用バイナリをビルドし、リリースを作成します。

#### リリースの仕組み

1. `main`または`master`ブランチにコミットをプッシュ
2. GitHub Actionsが自動的に：
   - Rustプロジェクトをビルド
   - バイナリをstripして最適化
   - Linux x86_64用バイナリをtar.gz形式でアーカイブ
   - SHA256チェックサムを生成
   - タイムスタンプとコミットハッシュを含むリリースを作成
   - GitHubのReleasesページに公開

3. リリース名の形式: `build-YYYYMMDD-HHMMSS-<commit-hash>`
   - 例: `build-20260606-120000-a1b2c3d4`

#### 最新ビルドの取得

[Releases](https://github.com/your-username/rss2tg/releases)ページから最新のビルドをダウンロードできます。各リリースには以下が含まれます：

- `rss2tg-linux-x86_64.tar.gz` - 実行可能バイナリ
- `rss2tg-linux-x86_64.tar.gz.sha256` - SHA256チェックサム

#### 手動でワークフローを実行

必要に応じて、GitHub ActionsのUIから手動でワークフローを実行することもできます：

1. GitHubリポジトリの「Actions」タブを開く
2. 「Release」ワークフローを選択
3. 「Run workflow」ボタンをクリック

### CI/CD設定

ワークフローファイル: [`.github/workflows/release.yml`](.github/workflows/release.yml)

- **トリガー**: `main`/`master`ブランチへのプッシュ、または手動実行
- **ビルド環境**: Ubuntu latest
- **キャッシュ**: Cargoレジストリ、インデックス、ビルド成果物
- **成果物**:
  - `rss2tg-linux-x86_64.tar.gz` - バイナリアーカイブ
  - `rss2tg-linux-x86_64.tar.gz.sha256` - チェックサムファイル

## ライセンス

このプロジェクトのライセンスについては、LICENSEファイルを参照してください。

## 貢献

プルリクエストを歓迎します。大きな変更の場合は、まずissueを開いて変更内容を議論してください。