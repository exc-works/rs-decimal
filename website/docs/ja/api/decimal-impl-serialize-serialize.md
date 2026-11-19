---
locale: ja
source_revision: b7aac8fb7a35df9ddf10adc50d88fb1bfd0169290aa22d5f4ad96531651b49c8
translation_provenance: en-US-japanese-reviewed
api_id: "Decimal::impl Serialize::serialize"
---
# Decimal::impl Serialize::serialize

Rust の Serde トレイトでシリアライズします。

```rust
fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error >
```

## 引数、戻り値、通常の使い方

serializer: Serializer を実装する S。S::Ok/S::Error を返します。コアは標準の小数文字列または null をシリアライズします。未初期化の Decimal は null、初期化済みのゼロは文字列 "0" になります。小数部分の末尾のゼロは除去します。ブラウザーの serialize() アダプターは serde_json を選び、完全な JSON テキストを返します。任意の Rust serializer は公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-serialize-serialize" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
