---
locale: zh-Hant
source_revision: c7dd2a0f7387f7110e7a059f6333707931d18a8d6fce4c06641505cdcb037d58
translation_provenance: zh-Hans-terminology-reviewed
api_id: "NullDecimal::impl Serialize::serialize"
---
# NullDecimal::impl Serialize::serialize

透過 Rust Serde trait 序列化。

```rust
fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error >
```

## 參數、回傳值及常規用法

serializer: 實現 Serializer 的 S；回傳 S::Ok/S::Error。核心序列化為規範小數字串或 null。無效 nullable 序列化為 null；內部零未初始化的有效值序列化為字串 "0"。移除小數末尾零。瀏覽器 serialize() 適配器選擇 serde_json，回傳完整 JSON 文字，不暴露任意 Rust serializer。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize-serialize" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
