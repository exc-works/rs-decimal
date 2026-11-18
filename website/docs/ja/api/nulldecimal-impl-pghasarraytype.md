---
locale: ja
source_revision: 329a8011aecf030884cb84a14d70c08abd0f17ed41927f957f56a638648169ac
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::impl PgHasArrayType"
---
# NullDecimal::impl PgHasArrayType

ネイティブの SQLx PostgreSQL NUMERIC 連携を使います。

```rust
impl PgHasArrayType for NullDecimal
```

## 通常の使い方と注意点

postgres 機能と SQLx 0.8 の PostgreSQL 型が必要です。この実装はネイティブ専用です。ブラウザーの playground はデータベース接続を開きません。array_type_info() は NUMERIC[] OID 1231 を返し、SQLx の配列バインディングを可能にします。NullDecimal の配列要素は有効性を通して SQL NULL を保持します。配列全体のエンコード／デコードにはネイティブ SQLx クエリーを使ってください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-pghasarraytype" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
