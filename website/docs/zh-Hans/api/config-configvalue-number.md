---
locale: zh-Hans
source_revision: d9b1450ea64f3f424b8c3303a77d21fe7ac3c0fd6d9a478841f3ae7df9627834
api_id: "config::ConfigValue::Number"
---
# config::ConfigValue::Number

选择 Number 配置输入。

```rust
Number (& 'a str)
```

## 参数、返回值及常规用法

传递数字源文本，不经过浮点转换。核心解析小数语法，此变体不是完整 JSON 语法验证器。空 Number 对 nullable 目标也无效。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-number" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
