---
locale: zh-Hans
source_revision: 5a5d502f0868797711c2e17ee061bb453dff465719822a493b3a33d226cf87b4
api_id: "config::ConfigValue::Nullable"
---
# config::ConfigValue::Nullable

选择 Nullable 配置输入。

```rust
Nullable (& 'a NullDecimal)
```

## 参数、返回值及常规用法

借用现有 nullable。decode_nullable 克隆有效标志和隐藏数据；decode_decimal 只接受有效值并克隆内部 Decimal。解码前的后续修改会被观察到。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-nullable" locale="zh-Hans" />

## 相关方法

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
