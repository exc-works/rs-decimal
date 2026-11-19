---
locale: zh-Hans
source_revision: 79f19f9f6f20dd385b69becd2b7e8e1abfeb536b960dc7d8b240d8301a6064de
api_id: "config::deserialize_json_nullable"
---
# config::deserialize_json_nullable

反序列化精确 JSON 配置字段。

```rust
fn deserialize_json_nullable < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < NullDecimal , D :: Error >
```

## 参数、返回值及常规用法

deserializer: 实现 Serde Deserializer 的 D；用于 #[serde(deserialize_with = "rs_decimal::config::deserialize_json_nullable")]. 需要 serde_json 源和 config feature。保留原始数字 token，不经过浮点数。浏览器提供具体适配器，参数为一个完整 JSON 标量源字符串，不是任意 deserializer。JSON null 和恰好为空的 JSON 字符串变为无效值；只有空白的字符串仍失败。成功的非空值变为有效。错误类型为 Serde deserializer 的错误；浏览器使用 category="serde" 保留消息，不虚构 DecimalError 类别。拒绝末尾非空白内容。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-nullable" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
