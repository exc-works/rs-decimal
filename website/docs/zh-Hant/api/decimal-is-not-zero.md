---
locale: zh-Hant
source_revision: 19e13f576512aa6ef6c12c54900338d87cb485da07e25d15d575e4c5353b491a
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::is_not_zero
---
# Decimal::is_not_zero

判斷數值是否非零。

```rust
fn is_not_zero (& self) -> bool
```

## 參數、回傳值及注意事項

回傳 bool/boolean，與 is_zero() 相反；未初始化的預設值回傳 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-not-zero" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
