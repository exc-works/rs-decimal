---
locale: ja
source_revision: 5f4867a89a40d93f24e8c716766cacdffb3d62078bd63369d4dd03f9f561ebeb
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::impl Debug"
---
# config::ConfigValue::impl Debug

ネイティブの Debug 実装を使います。

```rust
impl Debug for config::ConfigValue
```

## 引数、戻り値、通常の使い方

ConfigValue は小さな型付き列挙型です。Clone/Copy は参照先を深くコピーせず、借用参照を保持します。Debug は診断用テキストであり、安定した通信形式ではありません。ネイティブ Rust のトレイト構文は別に検証し、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-debug" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
