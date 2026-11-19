---
locale: zh-Hans
source_revision: 24e00c21cccbe1d37441ecf7320c640c608396a025aad09f2a518e2a7e914703
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

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
