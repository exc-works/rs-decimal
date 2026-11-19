---
locale: zh-Hant
source_revision: c40a135ffb56f404451bd244cc0aedf0eee2b3b5503190541e81212c60c34237
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_float32
---
# Decimal::new_from_float32

透過能往返還原浮點數的最短十進位表示建構。

```rust
fn new_from_float32 (value : f32) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

value: f32，以 JS number 傳入。JavaScript 先從 binary64 縮窄為 binary32，因此 16777217 變為 16777216。無法恢復之前因二進位浮點而丟失的十進位數字。精確十進位輸入請使用 new_from_string。NaN 和正負無窮回傳 InvalidFormat；有限 f64 縮窄為 f32 時溢出也回傳 InvalidFormat。負零變成已初始化且不帶符號的 Decimal 零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float32" locale="zh-Hant" />

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
