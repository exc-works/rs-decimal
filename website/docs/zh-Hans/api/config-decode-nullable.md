---
locale: zh-Hans
source_revision: 77287e6c4550f824e46a4556e0aaf001ccf92f4e78984f337df5b480f5f48874
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

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
