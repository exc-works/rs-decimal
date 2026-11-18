---
locale: ja
source_revision: 21da10a6e76a84c5146b6a083d73a886edce7505fb47206e66d109e57f693aeb
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl Decode < 'r , Postgres >::decode"
---
# Decimal::impl Decode < 'r , Postgres >::decode

ネイティブの SQLx PostgreSQL NUMERIC 連携を使います。

```rust
fn decode (value : PgValueRef < 'r >) -> Result < Self , BoxDynError >
```

## 通常の使い方と注意点

postgres 機能と SQLx 0.8 の PostgreSQL 型が必要です。この実装はネイティブ専用です。ブラウザーの playground はデータベース接続を開きません。decode(PgValueRef) は `Result<Self, BoxDynError>` を返します。SQLx が借用データベース値を提供するため、非公開の通信表現を自作しないでください。バイナリとテキストの NUMERIC は、スケールを持つゼロも含め、数値と表示スケールを保持します。SQL NULL は拒否します。nullable 列には NullDecimal または `Option<Decimal>` を使います。null 以外の結果は初期化済みです。不正な NUMERIC や非有限値は丸めずに拒否します。この例は CI の専用 PostgreSQL サーバーで実行します。ブラウザーに外部データベースは不要です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-decode-r-postgres-decode" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。専用の PostgreSQL サーバー、DATABASE_URL、`cargo run --locked -p decimal-web-examples --features postgres-live` でこの例を実行してください。このプロファイルがない場合は明示的にスキップと報告し、データベース検査の成功には数えません。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
