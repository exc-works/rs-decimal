---
locale: ja
source_revision: 385e2c824095efc3d2ce40587d64168818dba7b455d7f53f60a47309b6e48343
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl PgHasArrayType::array_type_info"
---
# Decimal::impl PgHasArrayType::array_type_info

ネイティブの SQLx PostgreSQL NUMERIC 連携を使います。

```rust
fn array_type_info () -> PgTypeInfo
```

## 通常の使い方と注意点

postgres 機能と SQLx 0.8 の PostgreSQL 型が必要です。この実装はネイティブ専用です。ブラウザーの playground はデータベース接続を開きません。array_type_info() は NUMERIC[] OID 1231 を返し、SQLx の配列バインディングを可能にします。NullDecimal の配列要素は有効性を通して SQL NULL を保持します。配列全体のエンコード／デコードにはネイティブ SQLx クエリーを使ってください。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-pghasarraytype-array-type-info" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
