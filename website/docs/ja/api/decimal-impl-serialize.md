---
locale: ja
source_revision: fd8204204097a2ca3064943a4cf9e6b865ae769705e2693bc4a79f71be2a4582
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl Serialize"
---
# Decimal::impl Serialize

Rust の Serde トレイトでシリアライズします。

```rust
impl Serialize for Decimal
```

## 引数、戻り値、通常の使い方

serializer: Serializer を実装する S。S::Ok/S::Error を返します。コアは標準の小数文字列または null をシリアライズします。未初期化の Decimal は null、初期化済みのゼロは文字列 "0" になります。小数部分の末尾のゼロは除去します。ブラウザーの serialize() アダプターは serde_json を選び、完全な JSON テキストを返します。任意の Rust serializer は公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-serialize" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
