---
locale: ja
source_revision: 324990894f6327d9f0e88937d867ea0f32bc3c792ba37210279bca0b73495f73
translation_provenance: en-US-japanese-reviewed
api_id: "config::decode_nullable"
---
# config::decode_nullable

任意の設定値をデコードします。

```rust
fn decode_nullable (value : ConfigValue < '_ >) -> Result < NullDecimal , DecimalError >
```

## 引数、戻り値、通常の使い方

value: ConfigValue。`Result<NullDecimal, DecimalError>`/新しい nullable ハンドルを返します。Null、厳密に空の Text、空の Bytes は無効になり、内部は未初期化のゼロです。空白だけの Text や空の Number は小数解析に失敗します。既存の Nullable は無効でも有効性と隠れた内部データの両方を複製します。借用 Decimal は強制的に初期化せず有効になります。他の入力は decode_decimal に従い、成功すると有効になります。入力は消費しません。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-nullable" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
