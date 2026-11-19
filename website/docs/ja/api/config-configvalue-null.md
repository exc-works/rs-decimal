---
locale: ja
source_revision: 846797abc20f334aac55a6cc16f18858d3e8f7870cd49af2d7f236e4e0d093b3
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Null"
---
# config::ConfigValue::Null

設定入力 Null を選択します。

```rust
Null
```

## 引数、戻り値、通常の使い方

存在しない設定入力です。decode_nullable は無効な値を返し、decode_decimal は Unmarshal で拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-null" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
