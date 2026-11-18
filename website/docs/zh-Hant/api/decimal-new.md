---
locale: zh-Hant
source_revision: fe9121bf755dd82c88fc1274dc43b09d6173d99c7a22441eb84bc6a73587fa71
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

## 用法注意事項和邊界

使用 `12n`，不能傳 JavaScript number。bridge 拒絕超出 i64 的值；小數輸入應使用 `new_from_string`。

瀏覽器還應用[資源限制](../resource-policy)。JavaScript 參數在呼叫 Rust 前進行類型及範圍檢查。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
