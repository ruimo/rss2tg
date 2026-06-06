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
- RSS Feed URL

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

Chat IDを取得するには、以下の手順を実行してください：

1. **Telegramで作成したBotを検索して開く**
   - Botのユーザー名（例：`@your_bot_name`）で検索
   - Botとのチャットを開く

2. **「START」ボタンを押して`/start`コマンドを実行**
   - 初回は必ず`/start`を実行する必要があります
   - これによりBotとのチャットが開始されます

3. **普通のテキストメッセージを送信**
   - 例：`Hello` や `Test` など、通常の文章を送信
   - `/start`だけではChat IDが取得できないため、この手順が必要です

4. **ブラウザで以下のURLにアクセス**
   ```
   https://api.telegram.org/bot<YOUR_BOT_TOKEN>/getUpdates
   ```
   - `<YOUR_BOT_TOKEN>`を実際のBot Tokenに置き換えてください

5. **レスポンスからChat IDを取得**
   
   レスポンス例：
   ```json
   {
     "ok": true,
     "result": [
       {
         "update_id": 123456789,
         "message": {
           "message_id": 2,
           "from": {
             "id": 987654321,
             "is_bot": false,
             "first_name": "Your Name"
           },
           "chat": {
             "id": 987654321,
             "first_name": "Your Name",
             "type": "private"
           },
           "date": 1234567890,
           "text": "Hello"
         }
       }
     ]
   }
   ```
   
   この例では、`"chat": { "id": 987654321 }`の`987654321`がChat IDです。

**トラブルシューティング:**
- レスポンスが`{"ok":true,"result":[]}`の場合：手順2と3を実行していません。`/start`を実行してから普通のテキストメッセージを送信してください。
- グループチャットで使用する場合：Botをグループチャットに追加してから同様の手順を実行してください（グループのChat IDは負の数になります）。

### 3. 環境変数の設定

以下の環境変数を設定してください：

```bash
export TELEGRAM_TOKEN="your_bot_token_here"
export CHAT_ID="your_chat_id_here"
export RSS_URL="https://example.com/rss"
```

または、`.env`ファイルを作成して管理することもできます（推奨）。

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
*/5 * * * * TELEGRAM_TOKEN="your_token" CHAT_ID="your_chat_id" RSS_URL="https://example.com/rss" /usr/local/bin/rss2tg
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
Environment="RSS_URL=https://example.com/rss"
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

### GitHub Actionsでの自動監視

GitHub Actionsを使用して、1時間ごとに自動的にRSSフィードを監視し、新しいエントリーをTelegramに通知できます。

#### 1. GitHubシークレットの設定

リポジトリにTelegram Bot Token、Chat ID、RSS URLを安全に保存します：

1. GitHubリポジトリページを開く
2. 「Settings」タブをクリック
3. 左サイドバーの「Secrets and variables」→「Actions」をクリック
4. 「New repository secret」ボタンをクリック
5. 以下の3つのシークレットを追加：

**シークレット1: TELEGRAM_TOKEN**
- Name: `TELEGRAM_TOKEN`
- Secret: あなたのTelegram Bot Token（例：`123456789:ABCdefGHIjklMNOpqrsTUVwxyz`）

**シークレット2: CHAT_ID**
- Name: `CHAT_ID`
- Secret: あなたのChat ID（例：`123456789`）

**シークレット3: RSS_URL**
- Name: `RSS_URL`
- Secret: 監視したいRSSフィードのURL（例：`https://example.com/rss`）

#### 2. ワークフローの有効化

[`.github/workflows/rss-monitor.yml`](.github/workflows/rss-monitor.yml)ワークフローが自動的に：

- **毎時0分（UTC時間）**にRSSフィードをチェック
- 最新のビルド済みバイナリを自動ダウンロード
- SQLiteデータベースをキャッシュして重複通知を防止
- 新しいエントリーがあればTelegramに通知

#### 3. 手動実行

必要に応じて手動で実行することもできます：

1. GitHubリポジトリの「Actions」タブを開く
2. 「RSS Monitor」ワークフローを選択
3. 「Run workflow」ボタンをクリック

#### 4. 実行スケジュールの変更

監視頻度を変更したい場合は、[`.github/workflows/rss-monitor.yml`](.github/workflows/rss-monitor.yml)の`cron`設定を編集してください：

```yaml
schedule:
  # 例：30分ごとに実行
  - cron: '*/30 * * * *'
  
  # 例：毎日9時（UTC）に実行
  - cron: '0 9 * * *'
```

**cron構文の例：**
- `0 * * * *` - 毎時0分
- `*/30 * * * *` - 30分ごと
- `0 */2 * * *` - 2時間ごと
- `0 9 * * *` - 毎日9:00 UTC

**注意**: GitHub Actionsの無料枠では、月2,000分まで利用可能です。1時間ごとの実行であれば十分に収まります。

**GitHub Actionsのスケジュール実行の制限**: 新しいリポジトリや活動の少ないリポジトリでは、スケジュール実行が正常に動作しない場合があります。その場合は、以下のJenkinsを使用した方法をお勧めします。

#### 5. 実行ログの確認

1. GitHubリポジトリの「Actions」タブを開く
2. 「RSS Monitor」ワークフローを選択
3. 実行履歴から確認したいrunをクリック
4. 各ステップの詳細ログを確認

### Jenkinsでの自動監視

GitHub Actionsのスケジュール実行が動作しない場合、Jenkinsを使用して定期実行できます。

#### 1. Jenkinsの準備

Jenkinsサーバーに以下の認証情報を設定します：

1. Jenkinsダッシュボードを開く
2. 「Manage Jenkins」→「Manage Credentials」をクリック
3. 適切なドメイン（例：Global）を選択
4. 「Add Credentials」をクリック
5. 以下の3つの認証情報を追加：

**認証情報1: telegram-token**
- Kind: Secret text
- Secret: あなたのTelegram Bot Token
- ID: `telegram-token`

**認証情報2: telegram-chat-id**
- Kind: Secret text
- Secret: あなたのChat ID
- ID: `telegram-chat-id`

#### 2. Jenkinsジョブの作成

1. Jenkinsダッシュボードで「New Item」をクリック
2. ジョブ名を入力（例：`rss2tg-monitor`）
3. 「Pipeline」を選択して「OK」をクリック
4. 「General」セクションで：
   - 「This project is parameterized」にチェック
   - 「Add Parameter」→「String Parameter」を選択
   - Name: `RSS_URL`
   - Default Value: 監視したいRSSフィードのURL（例：`https://example.com/rss`）
   - Description: `監視するRSSフィードのURL`
5. 「Pipeline」セクションで：
   - Definition: `Pipeline script from SCM`
   - SCM: `Git`
   - Repository URL: `https://github.com/ruimo/rss2tg.git`
   - Branch: `*/main`
   - Script Path: `Jenkinsfile`
6. 「Save」をクリック

#### 3. 実行スケジュール

[`Jenkinsfile`](Jenkinsfile)には以下のスケジュールが設定されています：

```groovy
triggers {
    // 毎時0分に実行
    cron('0 * * * *')
}
```

スケジュールを変更したい場合は、Jenkinsfileの`cron`設定を編集してください：

```groovy
// 例：30分ごとに実行
cron('*/30 * * * *')

// 例：毎日9時に実行
cron('0 9 * * *')
```

#### 4. 手動実行

必要に応じて手動で実行することもできます：

1. Jenkinsダッシュボードでジョブを選択
2. 「Build Now」をクリック

#### 5. 実行ログの確認

1. Jenkinsダッシュボードでジョブを選択
2. ビルド履歴から確認したいビルド番号をクリック
3. 「Console Output」をクリックして詳細ログを確認

#### Jenkinsfileの詳細

[`Jenkinsfile`](Jenkinsfile)は以下の処理を実行します：

1. **Setup**: データベース用ディレクトリを作成
2. **Download Binary**: GitHubから最新のリリースバイナリをダウンロード
3. **Run RSS Monitor**: RSS監視を実行
4. **Show Database Info**: データベースのサイズを表示
5. **Cleanup**: バイナリを削除（データベースは保持）

データベースファイル（`.rss-cache/rss_data.db`）はワークスペースに保持されるため、重複通知が防止されます。

## データベース

アプリケーションは`rss_data.db`というSQLiteデータベースファイルを作成し、通知済みのエントリーIDを記録します。これにより、同じエントリーが複数回通知されることを防ぎます。

## トラブルシューティング

### エラー: "environment variable not found"

環境変数`TELEGRAM_TOKEN`、`CHAT_ID`、または`RSS_URL`が設定されていません。上記のセットアップ手順を確認してください。

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