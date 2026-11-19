---
locale: zh-Hant
source_revision: 098d219e5e0932c9734847caf4da4f02238ac75ca2e27400d9527165d61283dd
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::cmp
---
# Decimal::cmp

用三態結果比較兩個數值。

```rust
fn cmp (& self , other : & Self) -> i32
```

## 參數、回傳值及注意事項

借用 other: &Decimal（JavaScript 中為 Decimal）。回傳 i32/number：-1、0 或 1。小數位數及初始化狀態不同不影響數值相等。這是固有方法，不是回傳 Ordering 的 Ord::cmp。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-cmp" locale="zh-Hant" />

## 相關方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
