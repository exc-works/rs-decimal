---
locale: zh-Hant
source_revision: 872ef2a43e7ac5d06f4ea120672471b0a862ee6f8a70e48db9f1f9367f2518f7
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::add_raw
---
# Decimal::add_raw

用 i64 整數改變未縮放的係數。

```rust
fn add_raw (& self , value : i64) -> Self
```

## 參數、回傳值及常規用法

value: i64，在 bridge 中使用 bigint。保留小數位數：在 scale 為 2 時，加減 1 改變數值 0.01，而非 1。Rust 回傳新 Decimal。bridge 拒絕 number 及超出 i64 範圍的 bigint。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="zh-Hant" />

## 瀏覽器邊界

輸入均被借用，成功和普通錯誤都不會消耗輸入。回傳的 Decimal 獨立擁有資料，算術結果已初始化。瀏覽器在呼叫核心前檢查 [資源限制](../resource-policy)：對齊、乘積中間係數、除法縮放及輸出精度。保守的預檢查可能拒絕最終結果很小的運算。資源錯誤與數值程式庫錯誤分開報告；panic 會終止執行。

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
