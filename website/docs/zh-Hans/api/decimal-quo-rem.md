---
locale: zh-Hans
source_revision: ab458b72e36df078d37dd6de4be21a55e10d1e6c955c47b4abcbe87cbbdd8e23
api_id: Decimal::quo_rem
---
# Decimal::quo_rem

返回整数商和带符号的 Decimal 余数。

```rust
fn quo_rem (& self , other : & Self) -> Result < (Self , Self) , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal；Rust 返回 Result<(Decimal, Decimal), DecimalError>，映射为 [quotient, remainder]。商向零截断且 scale 为零。余数与被除数同号，小数位数为两个输入中较大者，满足 self = quotient * other + remainder。每个结果都是独立且已初始化的对象；输入仍可使用。零除数在对齐前返回 DivideByZero。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-quo-rem" locale="zh-Hans" />

## 浏览器边界

遵循上述所有权规则。移动后的对象再次访问会返回 moved 错误；free 可重复调用。类型或别名检查在移动前执行，进入 Rust 后的错误不会恢复已移动的接收者。[资源限制](../resource-policy) 独立于库错误；幂运算检查中间值，对齐运算检查系数增长。保守检查可能拒绝最终结果很小的运算。

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
