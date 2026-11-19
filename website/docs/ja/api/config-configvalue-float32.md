---
locale: ja
source_revision: e064dade2acfdfd8dc8224a07d00162d5bdf94c66ca955bda753d76d5540ec7b
translation_provenance: en-US-japanese-reviewed
api_id: "config::ConfigValue::Float32"
---
# config::ConfigValue::Float32

設定入力 Float32 を選択します。

```rust
Float32 (f32)
```

## 引数、戻り値、通常の使い方

binary32 を保持します。Rust はまずブラウザーの number を f32 に狭め、コア変換を使います。非有限入力や変換時のオーバーフローは Unmarshal を返します。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float32" locale="ja" />

## 関連メソッド

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
