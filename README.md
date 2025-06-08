# ZennSlackMorphism

`slack-morphism` を使って Rust から Slack API でメッセージを送るサンプル

アプリで Block API でメッセージを送るだけならば Free プランでも可能

レートリミットはプラン問わず **チャンネルごと** に **1秒に1件** まで。
Burst 送信は可能だが長期間にわたって実行され続けると "429 Too Many Requests" になる。
<https://api.slack.com/apis/rate-limits#posting-messages>

## Prepare

<https://api.slack.com/apps> からアプリを作成する。
アプリの設定から "Oauth & Permissions" の Scopes から "Bot Token Scopes" で `chat:write` の権限を付与して `Bot User OAuth Token` ("xoxb-" から始まるキー) を生成する。

Slack のチャンネルの "インテグレーション" から "アプリを追加する" を押してアプリをチャンネルに紐づける。

"チャンネルID" をチャンネルの "チャンネル情報" の一番下にあるので取得する。

## Sample

### Block API

`Block API` を使って メッセージを送る。

```sh
SLACK_TOKEN= \
CHANNEL_ID= \
  cargo run --bin block
```

### Legacy

Lagacy な API を使って メッセージを送る。
基本的には Block API を使うこと。

```sh
SLACK_TOKEN= \
CHANNEL_ID= \
  cargo run --bin legacy
```
