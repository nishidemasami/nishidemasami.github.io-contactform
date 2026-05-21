# API

## 開発の開始方法

### Rustのインストール

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 環境変数の設定
```sh
. "$HOME/.cargo/env" 
```

## 各ソースの説明[WIP]
- lambda
  - Cargo.toml
  - src
    - bin
    - generate-openapi.rs
    - db.rs
    - handlers.rs
    - main.rs
    - models.rs
- openapi-develop.yaml
- openapi.yaml
- template.yaml

TODO:追記
