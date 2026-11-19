---
locale: ja
source_revision: db26ba148bd1b6fe560088f6be1bce313faff866656299f7c24eeb47568b46bf
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::impl Clone"
---
# config::ConfigValue::impl Clone

ネイティブの Clone 実装を使います。

```rust
impl Clone for config::ConfigValue
```

## 引数、戻り値、通常の使い方

ConfigValue は小さな型付き列挙型です。Clone/Copy は参照先を深くコピーせず、借用参照を保持します。Debug は診断用テキストであり、安定した通信形式ではありません。ネイティブ Rust のトレイト構文は別に検証し、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-clone" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
