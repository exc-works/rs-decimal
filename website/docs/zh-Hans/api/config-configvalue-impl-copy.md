---
locale: zh-Hans
source_revision: 39668a70027991d27730b01fd29253411ac796d1b8a82a794c00f60c687cf5f8
api_id: "config::ConfigValue::impl Copy"
---
# config::ConfigValue::impl Copy

使用原生 Copy 实现。

```rust
impl Copy for config::ConfigValue
```

## 参数、返回值及常规用法

ConfigValue 是小型类型枚举。Clone/Copy 保留借用引用，不深拷贝引用目标。Debug 是诊断文本，不是稳定的传输格式。原生 Rust trait 语法单独测试，不是浏览器方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-copy" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
