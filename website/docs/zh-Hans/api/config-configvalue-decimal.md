---
locale: zh-Hans
source_revision: 8d101e011ba2e4f441ebfc6371c73cb22589c6224599cc87e367af949b20dc8f
api_id: "config::ConfigValue::Decimal"
---
# config::ConfigValue::Decimal

选择 Decimal 配置输入。

```rust
Decimal (& 'a Decimal)
```

## 参数、返回值及常规用法

借用现有小数，在解码时克隆系数、scale 和初始化状态。源值仍可使用。浏览器源值在解码前释放或移动后，该借用变体不可用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-decimal" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
