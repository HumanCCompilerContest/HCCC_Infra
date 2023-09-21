# HCCC_Infra

![Logo](/Logo.png)

## 概要

人間 C コンパイラコンテスト(HCCC)とは文字通り競技者自身が C コンパイラとなり C 言語からアセンブリを生成し，その時間と正確さを競う競技です．

与えられるソースコードの中にはコンパイルエラーを出す必要の ある仕様上間違ったものも含まれています．このような場合にはコンパイルエラーと解答する必要があります.

## 起動方法

web_server, judge_server, test_runner, DB の構成です。

以下のコマンドで起動出来ます。

また，`.env.example`の環境変数をセットすることが出来ます。

```bash
# /
docker-compose pull test_runner
docker-compose up
```

※ 同時にフロントエンド側の起動も必要です。

## 使用技術

- Rust
- axum
- Docker
- postgres

## 開発者向け
以下のコマンドで開発向け環境を立ち上げることができます。

```bash
# 開発向け環境(ホットリロード，dbポート解放)
docker compose -f docker-compose.yaml -f docker-compose.local.yaml up
```

また、実際に提出物を実行するtest_runnerは、以下のコマンドでコンテナイメージ作成が行えます。

```bash
# 開発向け環境(ホットリロード，dbポート解放)
docker compose -f docker-compose.yaml -f docker-compose.local.yaml build test_runner
```

infrastructure for [HCCC](https://github.com/Alignof/Human_C_Compiler_Contest)
