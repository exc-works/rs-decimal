---
locale: zh-Hant
source_revision: c991ff009b87a451c4d3a01a7e163c4b8e995107fc18ca0a0ef4ec120e3aee71
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::min_ref
---
# Decimal::min_ref

按數值比較選擇原始運算元。

```rust
fn min_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## 參數、回傳值及常規用法

other: &Decimal；Rust 回傳與輸入生命週期關聯的共享參照。數值相等時選擇 self，保留精度和初始化狀態。JavaScript 回傳對應的原始物件，不複製。移動或釋放此物件會使它的所有 JS 別名失效；JS 不能強制 Rust 借用生命週期。需要獨立擁有的結果請使用 max/min。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-min-ref" locale="zh-Hant" />

## 相關方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
