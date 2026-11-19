---
locale: zh-Hans
source_revision: db26ba148bd1b6fe560088f6be1bce313faff866656299f7c24eeb47568b46bf
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

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
