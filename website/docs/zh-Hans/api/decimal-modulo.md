---
locale: zh-Hans
source_revision: 4f9cc4e1f27490d354a3bbc2ca44814e0fd0d846ebf81964d518e23cc81067c0
api_id: Decimal::modulo
---
# Decimal::modulo

返回向零截断除法的带符号余数。

```rust
fn modulo (& self , other : & Self) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal；Rust 返回 Result<Decimal, DecimalError>。余数与被除数同号，小数位数为两个输入中较大者；不是非负的欧几里得取模。零除数返回 DivideByZero。输入仅被借用。同时获取商和余数请使用 quo_rem。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-modulo" locale="zh-Hans" />

## 浏览器边界

遵循上述所有权规则。移动后的对象再次访问会返回 moved 错误；free 可重复调用。类型或别名检查在移动前执行，进入 Rust 后的错误不会恢复已移动的接收者。[资源限制](../resource-policy) 独立于库错误；幂运算检查中间值，对齐运算检查系数增长。保守检查可能拒绝最终结果很小的运算。

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
