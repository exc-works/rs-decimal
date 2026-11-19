---
locale: zh-Hans
source_revision: 1e9f449a346e8641c2e3739002dd283333ecd696e411549249e2395dfdddd2f0
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

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
