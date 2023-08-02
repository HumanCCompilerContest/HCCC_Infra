# HCCC_Infra

![Logo](/Logo.png)

## 概要

人間 C コンパイラコンテスト(HCCC)とは文字通り競技者自身が C コンパイラとなり C 言語からアセンブリを生成し，その時間と正確さを競う競技です．

与えられるソースコードの中にはコンパイルエラーを出す必要の ある仕様上間違ったものも含まれています．このような場合にはコンパイルエラーと解答する必要があります.

## 起動方法

web_server, judge_server, judge_container, DB の構成です。

```bash
# /
docker-compose up

# /web_server
cargo run

# /judge_server
cargo run
```

※ 同時にフロントエンド側の起動も必要です。

## 使用技術

- Rust
- axum
- Docker
- postgres

infrastructure for [HCCC](https://github.com/Alignof/Human_C_Compiler_Contest)
