---
locale: zh-Hans
source_revision: d858bd2326ab43e0cc593d11fb421e19dddaa88d9dec2ce79a679a247e556178
api_id: "config::ConfigValue::Float32"
---
# config::ConfigValue::Float32

选择 Float32 配置输入。

```rust
Float32 (f32)
```

## 参数、返回值及常规用法

传递 binary32。Rust 先将浏览器 number 收窄为 f32，再使用核心转换。非有限输入或收窄溢出返回 Unmarshal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float32" locale="zh-Hans" />

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
