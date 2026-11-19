---
locale: ja
source_revision: 39668a70027991d27730b01fd29253411ac796d1b8a82a794c00f60c687cf5f8
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::impl Copy"
---
# config::ConfigValue::impl Copy

ネイティブの Copy 実装を使います。

```rust
impl Copy for config::ConfigValue
```

## 引数、戻り値、通常の使い方

ConfigValue は小さな型付き列挙型です。Clone/Copy は参照先を深くコピーせず、借用参照を保持します。Debug は診断用テキストであり、安定した通信形式ではありません。ネイティブ Rust のトレイト構文は別に検証し、ブラウザーのメソッドではありません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-copy" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
