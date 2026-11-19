---
locale: ja
source_revision: 07611a32fa9c50a924b808d879544951b6661718e9f82991f25c28bfa7a925e8
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Signed"
---
# config::ConfigValue::Signed

設定入力 Signed を選択します。

```rust
Signed (i128)
```

## 引数、戻り値、通常の使い方

正確な符号付き i128 を保持します。ブラウザーは [-2^127, 2^127-1] の bigint を要求し、JS number と範囲外の値は変換前に拒否します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-signed" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
