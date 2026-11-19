---
locale: zh-Hans
source_revision: 9e6af4fbdd99e4ba366c644aaf4e403edbed7726d41f29fd990845c2a190132b
api_id: "config::ConfigValue::Unsigned"
---
# config::ConfigValue::Unsigned

选择 Unsigned 配置输入。

```rust
Unsigned (u128)
```

## 参数、返回值及常规用法

传递精确无符号 u128。浏览器要求 [0, 2^128-1] 范围的 bigint，不经过浮点数。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsigned" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
