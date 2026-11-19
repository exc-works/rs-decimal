---
locale: ja
source_revision: c7dd2a0f7387f7110e7a059f6333707931d18a8d6fce4c06641505cdcb037d58
translation_provenance: en-US-japanese-reviewed
api_id: "NullDecimal::impl Serialize::serialize"
---
# NullDecimal::impl Serialize::serialize

Rust の Serde トレイトでシリアライズします。

```rust
fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error >
```

## 引数、戻り値、通常の使い方

serializer: Serializer を実装する S。S::Ok/S::Error を返します。コアは標準の小数文字列または null をシリアライズします。無効な nullable は null、有効で内部が未初期化のゼロなら文字列 "0" になります。小数部分の末尾のゼロは除去します。ブラウザーの serialize() アダプターは serde_json を選び、完全な JSON テキストを返します。任意の Rust serializer は公開しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize-serialize" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
