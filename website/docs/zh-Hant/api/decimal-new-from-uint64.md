---
locale: zh-Hant
source_revision: a35340d9c55b8a08723d8ff0cfa4529bad65cad374d1045d7ce321047fc2a260
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_uint64
---
# Decimal::new_from_uint64

從無符號 64 位係數建立 Decimal，適合非負整數來源且小數位數單獨提供的場景。

```rust
fn new_from_uint64 (value : u64 , prec : isize) -> Result < Self , DecimalError >
```

## 參數和回傳值

`value: u64` 對應非負 `bigint`；`prec: isize` 對應整數 `number`，結果為 `value × 10^-prec`。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-uint64" locale="zh-Hant" />

## 用法注意事項和邊界

負值和超過 u64::MAX 的值會在進入 Rust 前被拒絕。負 precision 回傳程式庫錯誤 `InvalidPrecision`。JavaScript number 無法安全承載完整的 u64 範圍。

瀏覽器還應用[資源限制](../resource-policy)。JavaScript 參數在呼叫 Rust 前進行類型及範圍檢查。

## 相關方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
