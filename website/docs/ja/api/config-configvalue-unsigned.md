---
locale: ja
source_revision: 9e6af4fbdd99e4ba366c644aaf4e403edbed7726d41f29fd990845c2a190132b
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Unsigned"
---
# config::ConfigValue::Unsigned

設定入力 Unsigned を選択します。

```rust
Unsigned (u128)
```

## 引数、戻り値、通常の使い方

正確な符号なし u128 を保持します。ブラウザーは [0, 2^128-1] の bigint を要求し、浮動小数点を経由しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsigned" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
