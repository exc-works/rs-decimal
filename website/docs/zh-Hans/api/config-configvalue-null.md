---
locale: zh-Hans
source_revision: 846797abc20f334aac55a6cc16f18858d3e8f7870cd49af2d7f236e4e0d093b3
api_id: "config::ConfigValue::Null"
---
# config::ConfigValue::Null

选择 Null 配置输入。

```rust
Null
```

## 参数、返回值及常规用法

缺失的配置输入。decode_nullable 返回无效值，decode_decimal 返回 Unmarshal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-null" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
