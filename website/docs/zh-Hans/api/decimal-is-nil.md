---
locale: zh-Hans
source_revision: 7d0c030a450a67fa4b0e618dbc6cb501daeb5fbc64647ae39a02ca84be312b44
api_id: Decimal::is_nil
---
# Decimal::is_nil

判断 Decimal 是否为未初始化的默认值。

```rust
fn is_nil (& self) -> bool
```

## 参数、返回值及注意事项

返回 bool/boolean。默认值未初始化，构造得到的零已初始化，但两者数值相等。这也不是 NullDecimal 的有效性标记。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
