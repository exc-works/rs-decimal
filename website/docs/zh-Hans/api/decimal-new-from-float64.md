---
locale: zh-Hans
source_revision: 6c951b2ad37813f4b84e7dc136137d89576bf098308ff5b7d85a1bf0a3c58e09
api_id: Decimal::new_from_float64
---
# Decimal::new_from_float64

通过能往返还原浮点数的最短十进制表示构造。

```rust
fn new_from_float64 (value : f64) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

value: f64，以 JS number 传入。无法恢复之前因二进制浮点而丢失的十进制数字。精确十进制输入请使用 new_from_string。NaN 和正负无穷返回 InvalidFormat。负零变成已初始化且不带符号的 Decimal 零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float64" locale="zh-Hans" />

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
