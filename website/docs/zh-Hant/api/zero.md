---
locale: zh-Hant
source_revision: 68f642c8ee77ff168dce0e11e2443279259a34decdc0113ce303a8ad64b12c0c
translation_provenance: zh-Hans-terminology-reviewed
api_id: zero
---
# zero

建構已初始化且小數位數為零的零值。

```rust
fn zero () -> Decimal
```

## 參數、回傳值及注意事項

無參數，回傳新的 Decimal。與 Decimal::default() 不同，結果已初始化；兩者的數值仍然相等。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-zero" locale="zh-Hant" />

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
