---
locale: zh-Hans
source_revision: fa036d4ed1d96137decee6ee5f67e9b5d0d5b6d6fed77430c428472e7a62d1ff
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

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
