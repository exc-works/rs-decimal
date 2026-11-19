---
locale: zh-Hant
source_revision: 9f58b0f5c6b478fabd100e00297f6dfcdae4ff2a32ead701d8b208c279651c42
translation_provenance: zh-Hans-terminology-reviewed
api_id: min
---
# min

透過自由函數複製兩個 Decimal 中較小的一個。

```rust
fn min (a : & Decimal , b : & Decimal) -> Decimal
```

## 參數、回傳值及注意事項

借用 a 和 b: &Decimal，回傳獨立的 Decimal。數值相等時保留 a 的小數位數及初始化狀態，與 Decimal::min 的程式庫語義相同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-min" locale="zh-Hant" />

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
