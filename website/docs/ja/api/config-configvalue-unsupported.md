---
locale: ja
source_revision: dafd4579c57c7280a7d3e288044769361224c3db24eb3a6174eb3833dfe2bd19
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Unsupported"
---
# config::ConfigValue::Unsupported

設定入力 Unsupported を選択します。

```rust
Unsupported
```

## 引数、戻り値、通常の使い方

未対応の設定型を型変換せずに表します。両デコード関数は Unmarshal を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsupported" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
