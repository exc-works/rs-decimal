---
locale: zh-Hans
source_revision: b6da66eb1330fd7e72548ba45454893fec7b7bdb425b5a72d19b09887a6620cd
api_id: "config::ConfigValue::Unsigned"
---
# config::ConfigValue::Unsigned

选择 Unsigned 配置输入。

```rust
Unsigned (u128)
```

## 参数、返回值及常规用法

传递精确无符号 u128。浏览器要求 [0, 2^128-1] 范围的 bigint，不经过浮点数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsigned" locale="zh-Hans" />

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
