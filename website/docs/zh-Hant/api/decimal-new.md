---
locale: zh-Hant
source_revision: d578df6b6b71b8ce4b0e165f6e630705dc946bcefab55f8b43fc9cba6d60bb70
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

使用 `12n`，不能傳 JavaScript number。bridge 拒絕超出 i64 的值；小數輸入應使用 `new_from_string`。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
