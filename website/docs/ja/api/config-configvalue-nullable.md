---
locale: ja
source_revision: 5a5d502f0868797711c2e17ee061bb453dff465719822a493b3a33d226cf87b4
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Nullable"
---
# config::ConfigValue::Nullable

設定入力 Nullable を選択します。

```rust
Nullable (& 'a NullDecimal)
```

## 引数、戻り値、通常の使い方

既存の nullable を借用します。decode_nullable は有効性と隠れたデータを複製します。decode_decimal は有効な値のみを受け取り、その内部 Decimal を複製します。デコード前の変更は反映されます。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-nullable" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
