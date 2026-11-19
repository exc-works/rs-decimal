---
locale: zh-Hans
source_revision: 07611a32fa9c50a924b808d879544951b6661718e9f82991f25c28bfa7a925e8
api_id: "config::ConfigValue::Signed"
---
# config::ConfigValue::Signed

选择 Signed 配置输入。

```rust
Signed (i128)
```

## 参数、返回值及常规用法

传递精确有符号 i128。浏览器要求 [-2^127, 2^127-1] 范围的 bigint；转换前拒绝 JS number 和越界值。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-signed" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
