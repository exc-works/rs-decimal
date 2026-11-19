---
locale: zh-Hant
source_revision: f49d2dc4c074cf4f52b2a3a197eb1d13f1144942ab8a1df7ea4bf51caaaa350f
translation_provenance: zh-Hans-terminology-reviewed
api_id: max
---
# max

透過自由函數複製兩個 Decimal 中較大的一個。

```rust
fn max (a : & Decimal , b : & Decimal) -> Decimal
```

## 參數、回傳值及注意事項

借用 a 和 b: &Decimal，回傳獨立的 Decimal。數值相等時保留 a 的小數位數及初始化狀態，與 Decimal::max 的程式庫語義相同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-max" locale="zh-Hant" />

## 相關方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
