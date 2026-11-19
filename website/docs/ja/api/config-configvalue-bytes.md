---
locale: ja
source_revision: bfcaf340b0c1c1671610ce3a1025921897c59afd9bdf91da4be0b86f5f4d7446
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Bytes"
---
# config::ConfigValue::Bytes

設定入力 Bytes を選択します。

```rust
Bytes (& 'a [u8])
```

## 引数、戻り値、通常の使い方

小数のバイナリペイロードではなく、小数テキストを含むバイト列を借用します。ブラウザーではデコード前の Uint8Array の編集を反映し、デコード自体は配列を変更しません。不正な UTF-8 は小数解析に失敗します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-bytes" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
