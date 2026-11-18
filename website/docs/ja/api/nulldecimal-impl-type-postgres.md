---
locale: ja
source_revision: 0830b6acb470b67cab11063b94daf8eafba63a697871936bf2d04b61331f067d
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::impl Type < Postgres >"
---
# NullDecimal::impl Type < Postgres >

ネイティブの SQLx PostgreSQL NUMERIC 連携を使います。

```rust
impl Type < Postgres > for NullDecimal
```

## 通常の使い方と注意点

postgres 機能と SQLx 0.8 の PostgreSQL 型が必要です。この実装はネイティブ専用です。ブラウザーの playground はデータベース接続を開きません。type_info() は NUMERIC OID 1700 を返します。compatible(ty) はその OID だけを受理し、float8、整数、任意のテキスト型は受理しません。暗黙の浮動小数点変換は保証しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-nulldecimal-impl-type-postgres" locale="ja" />

## 実行

この項目はネイティブ Rust のトレイト構文であり、同名の JavaScript 演算子や SQLx オブジェクトを公開しません。既定のネイティブドキュメントテストでこの例を実行します。

## 関連メソッド

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
