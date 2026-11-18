---
locale: zh-Hans
source_revision: 4d2afeddcd523e94b9e9276fe6f8c19193bfe95254c1ef9ded67b301a566152b
api_id: Decimal::min_ref
---
# Decimal::min_ref

按数值比较选择原始操作数。

```rust
fn min_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## 参数、返回值及常规用法

other: &Decimal；Rust 返回与输入生命周期关联的共享引用。数值相等时选择 self，保留精度和初始化状态。bridge 返回对应的原始对象，不复制。移动或释放此对象会使它的所有 JS 别名失效；JS 不能强制 Rust 借用生命周期。需要独立拥有的结果请使用 max/min。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-min-ref" locale="zh-Hans" />

## 浏览器边界

遵循上述所有权规则。移动后的对象再次访问会返回 moved 错误；free 可重复调用。类型或别名检查在移动前执行，进入 Rust 后的错误不会恢复已移动的接收者。[资源限制](../resource-policy) 独立于库错误；幂运算检查中间值，对齐运算检查系数增长。保守检查可能拒绝最终结果很小的运算。

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
