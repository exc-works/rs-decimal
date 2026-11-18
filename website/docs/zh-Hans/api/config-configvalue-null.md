---
locale: zh-Hans
source_revision: 78ca92acc2a8523b4e668da7c55a57a56b16410b6455b88e11d916edee01152c
api_id: "config::ConfigValue::Null"
---
# config::ConfigValue::Null

选择 Null 配置输入。

```rust
Null
```

## 参数、返回值及常规用法

缺失的配置输入。decode_nullable 返回无效值，decode_decimal 返回 Unmarshal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-null" locale="zh-Hans" />

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
