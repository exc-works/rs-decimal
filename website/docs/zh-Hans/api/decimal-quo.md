---
locale: zh-Hans
source_revision: 0d6262b2eb7c6955f187e4d63b88d16a39d035f514ca55c738d74b10a8d69203
api_id: Decimal::quo
---
# Decimal::quo

以指定精度或输入精度相除。

```rust
fn quo (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal；Rust 返回 Result<Decimal, DecimalError>。输出小数位数为两个输入中较大者，因此两个整数输入也产生整数：1 / 3 使用 Down 得到 0，使用 Up 则得到 1。需要小数结果请使用 quo_with_prec。零除数返回 DivideByZero。mode 控制舍入；Unnecessary 对不精确结果返回 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
