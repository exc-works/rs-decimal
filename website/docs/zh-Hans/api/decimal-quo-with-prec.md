---
locale: zh-Hans
source_revision: eb9bcc338f948ddd5ec14bfd00bb5bb38695bb5e5eba9b781fe855d89569b0a0
api_id: Decimal::quo_with_prec
---
# Decimal::quo_with_prec

以指定精度或输入精度相除。

```rust
fn quo_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal；Rust 返回 Result<Decimal, DecimalError>。prec 指定非负的小数位数；负精度在检查除数前返回 InvalidPrecision。零除数返回 DivideByZero。mode 控制舍入；Unnecessary 对不精确结果返回 RoundUnnecessary。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-with-prec" locale="zh-Hans" />

## 相关方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
