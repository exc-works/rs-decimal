---
locale: zh-Hans
source_revision: 5bb3ef8c2d718846f6ac7a4004c6720c37095a64615189ff33906f2fba479837
api_id: "config::ConfigValue::Signed"
---
# config::ConfigValue::Signed

选择 Signed 配置输入。

```rust
Signed (i128)
```

## 参数、返回值及常规用法

传递精确有符号 i128。浏览器要求 [-2^127, 2^127-1] 范围的 bigint；转换前拒绝 JS number 和越界值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-signed" locale="zh-Hans" />

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
