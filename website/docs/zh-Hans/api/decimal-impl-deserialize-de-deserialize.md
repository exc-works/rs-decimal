---
locale: zh-Hans
source_revision: 40f2750ccc6afed4e917ce4038d5c5a19643a90b97f7264c770c82c1da409a91
api_id: "Decimal::impl Deserialize < 'de >::deserialize"
---
# Decimal::impl Deserialize < 'de >::deserialize

通过通用 Serde visitor 反序列化。

```rust
fn deserialize < D : Deserializer < 'de > > (d : D) -> Result < Self , D :: Error >
```

## 参数、返回值及常规用法

deserializer: 实现 Deserializer 的 D；返回 Self 或 D::Error。接受精确小数字符串、整数 visitor 输入和 null。拒绝浮点 visitor 输入：JSON 小数或指数数字应使用专门的原始 JSON/config 适配器。JSON null 产生未初始化的 Decimal 零。空字符串无效。浏览器静态 deserialize(source) 适配器对完整原始 JSON 文本执行 serde_json，范围小于任意 Rust deserializer。错误提供 category="serde" 和原消息。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-deserialize-de-deserialize" locale="zh-Hans" />

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
