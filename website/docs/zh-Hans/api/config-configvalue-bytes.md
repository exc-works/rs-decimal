---
locale: zh-Hans
source_revision: bfcaf340b0c1c1671610ce3a1025921897c59afd9bdf91da4be0b86f5f4d7446
api_id: "config::ConfigValue::Bytes"
---
# config::ConfigValue::Bytes

选择 Bytes 配置输入。

```rust
Bytes (& 'a [u8])
```

## 参数、返回值及常规用法

借用小数文本字节，不是二进制小数负载。浏览器观察解码前的 Uint8Array 修改，解码不修改数组。无效 UTF-8 导致小数解析失败。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-bytes" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
