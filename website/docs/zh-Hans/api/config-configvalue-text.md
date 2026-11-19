---
locale: zh-Hans
source_revision: 89abd084e8bb8541133194e2046ba4621ebe68fb1c342334ebc3bf5d1ca7dcec
api_id: "config::ConfigValue::Text"
---
# config::ConfigValue::Text

选择 Text 配置输入。

```rust
Text (& 'a str)
```

## 参数、返回值及常规用法

借用精确小数文本。首尾空白由小数核心解析，不移除 SQL 风格外围引号。仅恰好为空的文本在 nullable 解码时表示缺失。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-text" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
