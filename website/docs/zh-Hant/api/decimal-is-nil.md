---
locale: zh-Hant
source_revision: 7d0c030a450a67fa4b0e618dbc6cb501daeb5fbc64647ae39a02ca84be312b44
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::is_nil
---
# Decimal::is_nil

判斷 Decimal 是否為未初始化的預設值。

```rust
fn is_nil (& self) -> bool
```

## 參數、回傳值及注意事項

回傳 bool/boolean。預設值未初始化，建構得到的零已初始化，但兩者數值相等。這也不是 NullDecimal 的有效性標記。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
