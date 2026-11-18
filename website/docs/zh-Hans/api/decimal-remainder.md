---
locale: zh-Hans
source_revision: d4492d82b410cc5d7b67274496227b1fa1d6279b6d276d6bd8bfbe25ef422dd4
api_id: Decimal::remainder
---
# Decimal::remainder

拆分整数部分和未缩放的小数余数。

```rust
fn remainder (& self) -> (BigInt , BigInt)
```

## 参数、返回值及常规用法

无参数；返回 (BigInt, BigInt)，映射为 [bigint, bigint]。第一项向零截断；第二项是带符号的系数余数，不是 Decimal 小数。-12.750 得到 (-12, -750)：第二项需要原始 scale 3 才表示 -0.750。默认零返回 (0, 0)。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-remainder" locale="zh-Hans" />

## 浏览器边界

遵循上述所有权规则。移动后的对象再次访问会返回 moved 错误；free 可重复调用。类型或别名检查在移动前执行，进入 Rust 后的错误不会恢复已移动的接收者。[资源限制](../resource-policy) 独立于库错误；幂运算检查中间值，对齐运算检查系数增长。保守检查可能拒绝最终结果很小的运算。

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
