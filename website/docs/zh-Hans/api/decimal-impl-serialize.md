---
locale: zh-Hans
source_revision: fd8204204097a2ca3064943a4cf9e6b865ae769705e2693bc4a79f71be2a4582
api_id: "Decimal::impl Serialize"
---
# Decimal::impl Serialize

通过 Rust Serde trait 序列化。

```rust
impl Serialize for Decimal 
```

## 参数、返回值及常规用法

serializer: 实现 Serializer 的 S；返回 S::Ok/S::Error。核心序列化为规范小数字符串或 null。未初始化 Decimal 序列化为 null，已初始化零序列化为字符串 "0"。移除小数末尾零。浏览器 serialize() 适配器选择 serde_json，返回完整 JSON 文本，不暴露任意 Rust serializer。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-serialize" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
