---
locale: zh-Hans
source_revision: 48f0114735b7fdf69a527f5e2e6dcf6a17a2e69d41b0a53a9ff65fb0ac1362fd
api_id: "NullDecimal::impl Serialize"
---
# NullDecimal::impl Serialize

通过 Rust Serde trait 序列化。

```rust
impl Serialize for NullDecimal 
```

## 参数、返回值及常规用法

serializer: 实现 Serializer 的 S；返回 S::Ok/S::Error。核心序列化为规范小数字符串或 null。无效 nullable 序列化为 null；内部零未初始化的有效值序列化为字符串 "0"。移除小数末尾零。浏览器 serialize() 适配器选择 serde_json，返回完整 JSON 文本，不暴露任意 Rust serializer。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
