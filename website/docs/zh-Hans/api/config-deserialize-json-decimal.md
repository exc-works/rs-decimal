---
locale: zh-Hans
source_revision: a26edb51e2fbe0ae6bac5abd8ce3e12bd13315bb3e836a0d554d5f19af0430ce
api_id: "config::deserialize_json_decimal"
---
# config::deserialize_json_decimal

反序列化精确 JSON 配置字段。

```rust
fn deserialize_json_decimal < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < Decimal , D :: Error >
```

## 参数、返回值及常规用法

deserializer: 实现 Serde Deserializer 的 D；用于 #[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")]. 需要 serde_json 源和 config feature。保留原始数字 token，不经过浮点数。浏览器提供具体适配器，参数为一个完整 JSON 标量源字符串，不是任意 deserializer。不可空目标拒绝 JSON null 和空字符串。错误类型为 Serde deserializer 的错误；浏览器使用 category="serde" 保留消息，不虚构 DecimalError 类别。拒绝末尾非空白内容。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-decimal" locale="zh-Hans" />

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
