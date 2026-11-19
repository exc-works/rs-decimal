---
locale: zh-Hans
source_revision: 032d932903a81f0f03e2bde5c0ffb35c8a1064335f6ef0a3149d91077fa81075
api_id: "config::ConfigValue::Float64"
---
# config::ConfigValue::Float64

选择 Float64 配置输入。

```rust
Float64 (f64)
```

## 参数、返回值及常规用法

通过核心最短十进制转换传递 binary64。非有限数返回 Unmarshal，已因浮点舍入丢失的数字无法恢复。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float64" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
