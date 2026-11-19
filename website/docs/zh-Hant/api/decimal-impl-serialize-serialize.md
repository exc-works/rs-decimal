---
locale: zh-Hant
source_revision: b7aac8fb7a35df9ddf10adc50d88fb1bfd0169290aa22d5f4ad96531651b49c8
translation_provenance: zh-Hans-terminology-reviewed
api_id: "Decimal::impl Serialize::serialize"
---
# Decimal::impl Serialize::serialize

透過 Rust Serde trait 序列化。

```rust
fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error >
```

## 參數、回傳值及常規用法

serializer: 實現 Serializer 的 S；回傳 S::Ok/S::Error。核心序列化為規範小數字串或 null。未初始化 Decimal 序列化為 null，已初始化零序列化為字串 "0"。移除小數末尾零。瀏覽器 serialize() 適配器選擇 serde_json，回傳完整 JSON 文字，不暴露任意 Rust serializer。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-serialize-serialize" locale="zh-Hant" />

## 相關方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
