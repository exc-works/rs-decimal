---
locale: zh-Hans
source_revision: 34aa17297c2ce6b5b73e997c910f57874da73c70f9cf52cd3fadd2ab710a529d
api_id: Decimal::try_from_coefficient
---
# Decimal::try_from_coefficient

通过拥有所有权的系数和小数位数构造已初始化值。

```rust
fn try_from_coefficient (coefficient : BigInt , scale : usize) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

coefficient: BigInt；scale: usize（非负小数位数）。保留末尾零和零值的精度，不使用文本解析器的规范化。Rust 在超过 isize::MAX 时返回 Overflow。浏览器使用精确 bigint 和 wasm32 usize 范围内的整数 number；超过 i32::MAX 的 scale 保留 Overflow，其他超过 1000 的 scale 触及 playground 资源限制。JS bigint 不可变，因此不是被移动的包装对象。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-try-from-coefficient" locale="zh-Hans" />

## 浏览器边界

遵循上述所有权规则。移动后的对象再次访问会返回 moved 错误；free 可重复调用。类型或别名检查在移动前执行，进入 Rust 后的错误不会恢复已移动的接收者。[资源限制](../resource-policy) 独立于库错误；幂运算检查中间值，对齐运算检查系数增长。保守检查可能拒绝最终结果很小的运算。

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
