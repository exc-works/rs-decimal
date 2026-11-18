---
locale: zh-Hans
source_revision: e777d7a0d2eada54e45b9094274df294b6fb9f6fd490746dfc2797aa8d3acb40
api_id: "config::ConfigValue::impl Clone"
---
# config::ConfigValue::impl Clone

使用原生 Clone 实现。

```rust
impl Clone for config::ConfigValue
```

## 参数、返回值及常规用法

ConfigValue 是小型类型枚举。Clone/Copy 保留借用引用，不深拷贝引用目标。Debug 是诊断文本，不是稳定的传输格式。原生 Rust trait 语法单独测试，不是浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-clone" locale="zh-Hans" />

## 浏览器边界

config 与 serde feature 已启用。源文本和字节最多 8192 个 UTF-8 字节；Rust 在解析前检查小数文本和 JSON 解码后的字符串，因此转义不能绕过 [资源限制](../resource-policy)。128 位整数必须使用范围内的 bigint。借用的 Decimal/NullDecimal 在解码时必须仍有效，转换生成独立值，不消耗输入。Serde 的通用 trait 语法由具体 JSON 适配器演示。

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
