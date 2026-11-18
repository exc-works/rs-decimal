---
locale: zh-Hans
source_revision: 9cc3411fb84577d5ea175fe7b4b5d7e0d13f86f3ba54a79c3a0728b721c31211
api_id: "config::ConfigValue::Text"
---
# config::ConfigValue::Text

选择 Text 配置输入。

```rust
Text (& 'a str)
```

## 参数、返回值及常规用法

借用精确小数文本。首尾空白由小数核心解析，不移除 SQL 风格外围引号。仅恰好为空的文本在 nullable 解码时表示缺失。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-text" locale="zh-Hans" />

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
