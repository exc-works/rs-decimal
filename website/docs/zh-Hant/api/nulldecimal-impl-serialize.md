---
locale: zh-Hant
source_revision: 48f0114735b7fdf69a527f5e2e6dcf6a17a2e69d41b0a53a9ff65fb0ac1362fd
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::impl Serialize"
---
# NullDecimal::impl Serialize

透過 Rust Serde trait 序列化。

```rust
impl Serialize for NullDecimal
```

## 參數、回傳值及常規用法

serializer: 實現 Serializer 的 S；回傳 S::Ok/S::Error。核心序列化為規範小數字串或 null。無效 nullable 序列化為 null；內部零未初始化的有效值序列化為字串 "0"。移除小數末尾零。瀏覽器 serialize() 適配器選擇 serde_json，回傳完整 JSON 文字，不暴露任意 Rust serializer。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
