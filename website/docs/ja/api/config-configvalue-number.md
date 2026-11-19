---
locale: ja
source_revision: d9b1450ea64f3f424b8c3303a77d21fe7ac3c0fd6d9a478841f3ae7df9627834
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Number"
---
# config::ConfigValue::Number

設定入力 Number を選択します。

```rust
Number (& 'a str)
```

## 引数、戻り値、通常の使い方

浮動小数点へ変換せず、数値のソーステキストを保持します。コアは小数構文を解析します。このバリアントは完全な JSON 文法検証器ではありません。空の Number は nullable の対象でも不正です。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-number" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
