---
locale: zh-Hans
source_revision: 324990894f6327d9f0e88937d867ea0f32bc3c792ba37210279bca0b73495f73
api_id: "config::decode_nullable"
---
# config::decode_nullable

解码可空配置值。

```rust
fn decode_nullable (value : ConfigValue < '_ >) -> Result < NullDecimal , DecimalError >
```

## 参数、返回值及常规用法

value: ConfigValue；返回 `Result<NullDecimal, DecimalError>`/新的可空句柄。Null、恰好为空的 Text 和空 Bytes 产生无效值，内部零未初始化。只有空白的 Text 和空 Number 仍会解析失败。现有 Nullable 克隆有效标志和隐藏内部数据，即使原值无效；借用 Decimal 时结果有效，但不会强制初始化。其他输入遵循 decode_decimal，成功后变为有效。输入不被消耗。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-nullable" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
