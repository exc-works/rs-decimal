---
locale: zh-Hant
source_revision: 6c951b2ad37813f4b84e7dc136137d89576bf098308ff5b7d85a1bf0a3c58e09
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_float64
---
# Decimal::new_from_float64

透過能往返還原浮點數的最短十進位表示建構。

```rust
fn new_from_float64 (value : f64) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

value: f64，以 JS number 傳入。無法恢復之前因二進位浮點而丟失的十進位數字。精確十進位輸入請使用 new_from_string。NaN 和正負無窮回傳 InvalidFormat。負零變成已初始化且不帶符號的 Decimal 零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float64" locale="zh-Hant" />

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
