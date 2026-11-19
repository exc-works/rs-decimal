---
locale: zh-Hant
source_revision: 536eae84e3e9c7c0c702cbcb339f82c64bf0aafab561d4a3f87590939cb52d8e
translation_provenance: zh-Hans-terminology-reviewed
api_id: between
---
# between

判斷 Decimal 是否位於包含兩端點的數值區間內。

```rust
fn between (v : & Decimal , lower : & Decimal , upper : & Decimal) -> bool
```

## 參數、回傳值及注意事項

借用 v、lower 和 upper: &Decimal；回傳 bool/boolean，判斷 lower <= v <= upper。相等按數值判斷，忽略表示差異。不會重排邊界，lower > upper 時回傳 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-between" locale="zh-Hant" />

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
