---
locale: ja
source_revision: 8d101e011ba2e4f441ebfc6371c73cb22589c6224599cc87e367af949b20dc8f
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Decimal"
---
# config::ConfigValue::Decimal

設定入力 Decimal を選択します。

```rust
Decimal (& 'a Decimal)
```

## 引数、戻り値、通常の使い方

既存の小数を借用し、デコード時に係数、スケール、初期化状態を複製します。元の値は引き続き使用できます。ブラウザーでデコード前に元の値を解放またはムーブすると、この借用バリアントは無効になります。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-decimal" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
