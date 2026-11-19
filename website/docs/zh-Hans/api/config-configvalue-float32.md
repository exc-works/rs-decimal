---
locale: zh-Hans
source_revision: e064dade2acfdfd8dc8224a07d00162d5bdf94c66ca955bda753d76d5540ec7b
api_id: "config::ConfigValue::Float32"
---
# config::ConfigValue::Float32

选择 Float32 配置输入。

```rust
Float32 (f32)
```

## 参数、返回值及常规用法

传递 binary32。Rust 先将浏览器 number 收窄为 f32，再使用核心转换。非有限输入或收窄溢出返回 Unmarshal。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float32" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
