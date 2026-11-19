---
locale: zh-Hant
source_revision: 05d4c976c4f23a3286f06a831a12b99e1cda6e2ba4014722196a6ec9aade2d22
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::equal
---
# Decimal::equal

判斷兩個 Decimal 的數值是否相同。

```rust
fn equal (& self , other : & Self) -> bool
```

## 參數、回傳值及注意事項

借用 other: &Decimal，回傳 bool/boolean，忽略小數位數及初始化狀態。JavaScript 的 === 比較包裝物件身份；數值相等應呼叫 equal()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-equal" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
