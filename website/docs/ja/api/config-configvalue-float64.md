---
locale: ja
source_revision: 032d932903a81f0f03e2bde5c0ffb35c8a1064335f6ef0a3149d91077fa81075
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Float64"
---
# config::ConfigValue::Float64

設定入力 Float64 を選択します。

```rust
Float64 (f64)
```

## 引数、戻り値、通常の使い方

コアの最短小数変換を通して binary64 を保持します。非有限値は Unmarshal を返します。浮動小数点の丸めですでに失われた桁は復元できません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float64" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
