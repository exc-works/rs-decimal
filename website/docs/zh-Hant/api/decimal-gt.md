---
locale: zh-Hant
source_revision: f3c9953fc7c2ab0c097e714a9d8217e98239ea7e625d7a02d8c88afee82038af
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::gt
---
# Decimal::gt

判斷目前數值是否大於另一個數值。

```rust
fn gt (& self , other : & Self) -> bool
```

## 參數、回傳值及注意事項

借用 other: &Decimal，回傳 bool/boolean。數值相等時回傳 false，即使儲存的小數位數不同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gt" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
