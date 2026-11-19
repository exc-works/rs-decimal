---
locale: zh-Hant
source_revision: 3a7d890770468c269032ca95ee3365a1f5fdeb70051590bf674c733d1e1658d6
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::is_zero
---
# Decimal::is_zero

在後續操作前判斷數值是否為零。

```rust
fn is_zero (& self) -> bool
```

## 參數、回傳值及注意事項

回傳 bool/boolean，未初始化的預設值也回傳 true。需要區分初始化狀態時使用 is_nil()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
