---
locale: zh-Hans
source_revision: ddfedf2b2a7b0914238752b884265b0c43b55c647cf39e728c8143e706a5b5ba
api_id: Decimal::sub_owned
---
# Decimal::sub_owned

消耗左操作数进行精确运算。

```rust
fn sub_owned (self , other : & Self) -> Self
```

## 参数、返回值及常规用法

other: &Decimal；返回小数位数为两个输入中较大者的已初始化 Decimal。接收者被移动，other 仍可使用。可能复用存储，但增长仍会分配。bridge 在移动前拒绝同一对象同时作为接收者和 other。进入 Rust 后，即使资源错误也会消耗接收者。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-sub-owned" locale="zh-Hans" />

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
