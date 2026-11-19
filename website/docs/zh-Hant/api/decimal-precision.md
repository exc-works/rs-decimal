---
locale: zh-Hant
source_revision: 0ad97f33b356cd281b88951f90d8d3c10dff06f728f22f7136b85c34bc769e4e
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::precision
---
# Decimal::precision

讀取數值儲存的小數位數。

```rust
fn precision (& self) -> isize
```

## 參數、回傳值及注意事項

回傳 isize，在瀏覽器中為整數 number。這是小數位數，不是有效位數位數；末尾零也計入其中。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
