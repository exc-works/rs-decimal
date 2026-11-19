---
locale: zh-Hans
source_revision: 3594148ee83ab0bf68ef800a2a73612702e5a4d6ff495df20f526a7cdddcfcf1
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

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
