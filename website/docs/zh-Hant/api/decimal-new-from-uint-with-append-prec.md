---
locale: zh-Hant
source_revision: 2f8f701187893878a876b218458ba92b8464c997083c08e62c584ef14b118454
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_uint_with_append_prec
---
# Decimal::new_from_uint_with_append_prec

保持無符號整數的數值不變，同時追加儲存的小數位數。

```rust
fn new_from_uint_with_append_prec (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## 參數和回傳值

`value: u64` 對應非負 `bigint`；`prec: isize` 對應整數 `number`，結果保持原整數值並採用 scale `prec`。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint-with-append-prec" locale="zh-Hant" />

## 用法注意事項和邊界

負值和超出範圍的值屬於 bridge 參數錯誤。負 precision 回傳 `InvalidPrecision`；過大 precision 觸及獨立的 Playground 資源限制。請透過保留尾零的輸出查看追加的小數位數。

瀏覽器還應用[資源限制](../resource-policy)。JavaScript 參數在呼叫 Rust 前進行類型及範圍檢查。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
