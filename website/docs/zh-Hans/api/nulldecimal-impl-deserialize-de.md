---
locale: zh-Hans
source_revision: 22a6d48f36f7716bfae657ccf09549a7c182a92cb0e3afef6f24a947535edb2b
api_id: "NullDecimal::impl Deserialize < 'de >"
---
# NullDecimal::impl Deserialize < 'de >

通过通用 Serde visitor 反序列化。

```rust
impl< 'de > Deserialize < 'de > for NullDecimal 
```

## 参数、返回值及常规用法

deserializer: 实现 Deserializer 的 D；返回 Self 或 D::Error。接受精确小数字符串、整数 visitor 输入和 null。拒绝浮点 visitor 输入：JSON 小数或指数数字应使用专门的原始 JSON/config 适配器。JSON null 产生无效 nullable，内部零未初始化。空字符串无效。浏览器静态 deserialize(source) 适配器对完整原始 JSON 文本执行 serde_json，范围小于任意 Rust deserializer。错误提供 category="serde" 和原消息。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-deserialize-de" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
