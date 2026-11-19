---
locale: ja
source_revision: 48f0114735b7fdf69a527f5e2e6dcf6a17a2e69d41b0a53a9ff65fb0ac1362fd
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::impl Serialize"
---
# NullDecimal::impl Serialize

Rust の Serde トレイトでシリアライズします。

```rust
impl Serialize for NullDecimal
```

## 引数、戻り値、通常の使い方

serializer: Serializer を実装する S。S::Ok/S::Error を返します。コアは標準の小数文字列または null をシリアライズします。無効な nullable は null、有効で内部が未初期化のゼロなら文字列 "0" になります。小数部分の末尾のゼロは除去します。ブラウザーの serialize() アダプターは serde_json を選び、完全な JSON テキストを返します。任意の Rust serializer は公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
