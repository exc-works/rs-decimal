---
locale: zh-Hant
source_revision: 3e32f687004e72286fa606992f7977ccbc210f3d36756c6006ab536cd5091f5a
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_decimal
---
# Decimal::new_from_decimal

複製現有 Decimal，同時保留其儲存表示和初始化狀態。

```rust
fn new_from_decimal (value : & Self) -> Self
```

## 參數和回傳值

借用 `value: &Decimal`，回傳獨立的 Decimal；輸入仍可繼續使用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-decimal" locale="zh-Hant" />

## 使用說明

複製 `Decimal::default()` 會保留未初始化狀態，不會變成已初始化的零。JavaScript 必須傳入同一次執行中仍有效的 Decimal；此建構器不消耗它。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
