---
locale: zh-Hans
source_revision: db43c880650b6eb59e9c805ec0cdb77a4879108bb98a5d2e10ec38f65ac55f46
api_id: Decimal::sub
---
# Decimal::sub

精确相减。

```rust
fn sub (& self , other : & Self) -> Self
```

## 参数、返回值及常规用法

other: &Decimal；返回小数位数为两个输入中较大者的新 Decimal。对齐是精确的，不丢弃数字。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
