---
locale: zh-Hans
source_revision: c40a135ffb56f404451bd244cc0aedf0eee2b3b5503190541e81212c60c34237
api_id: Decimal::new_from_float32
---
# Decimal::new_from_float32

通过能往返还原浮点数的最短十进制表示构造。

```rust
fn new_from_float32 (value : f32) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

value: f32，以 JS number 传入。JavaScript 先从 binary64 缩窄为 binary32，因此 16777217 变为 16777216。无法恢复之前因二进制浮点而丢失的十进制数字。精确十进制输入请使用 new_from_string。NaN 和正负无穷返回 InvalidFormat；有限 f64 缩窄为 f32 时溢出也返回 InvalidFormat。负零变成已初始化且不带符号的 Decimal 零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-float32" locale="zh-Hans" />

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
