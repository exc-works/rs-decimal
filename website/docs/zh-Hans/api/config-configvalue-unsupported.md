---
locale: zh-Hans
source_revision: dafd4579c57c7280a7d3e288044769361224c3db24eb3a6174eb3833dfe2bd19
api_id: "config::ConfigValue::Unsupported"
---
# config::ConfigValue::Unsupported

选择 Unsupported 配置输入。

```rust
Unsupported
```

## 参数、返回值及常规用法

明确表示不支持的配置类型，不做强制转换。两个 decode 函数都返回 Unmarshal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsupported" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
