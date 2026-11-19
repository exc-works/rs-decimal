---
locale: zh-Hans
source_revision: 181ceff066af430ef0b4f061e0d4890d14b967a20ab1d2fe96ac03e3b4f024fe
api_id: Decimal::add_owned
---
# Decimal::add_owned

消耗左操作数进行精确运算。

```rust
fn add_owned (self , other : & Self) -> Self
```

## 参数、返回值及常规用法

other: &Decimal；返回小数位数为两个输入中较大者的已初始化 Decimal。接收者被移动，other 仍可使用。可能复用存储，但增长仍会分配。JavaScript 在移动前拒绝同一对象同时作为接收者和 other。进入 Rust 后，即使资源错误也会消耗接收者。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-add-owned" locale="zh-Hans" />

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
