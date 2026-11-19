---
locale: zh-Hant
source_revision: 94e5c200ef4c48b600cdc1b46dd606d6ca8e9e2741aedbb2bac50f53ac53727e
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new
---
# Decimal::new

建立已初始化的整數，適合有符號 64 位計數或整數金額。

```rust
fn new (value : i64) -> Self
```

## 參數和回傳值

`value: i64` 對應 JavaScript `bigint`，結果的 scale 為零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new" locale="zh-Hant" />

## 使用說明

使用 `12n`，不能傳 JavaScript number。JavaScript 拒絕超出 i64 的值；小數輸入應使用 `new_from_string`。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
