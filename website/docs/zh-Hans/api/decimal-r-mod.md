---
locale: zh-Hans
source_revision: 68a6c601d819a031d04a43cce835c2569811cfe3ffa0049911e2723c800f6029
api_id: Decimal::r#mod
---
# Decimal::r#mod

返回向零截断除法的带符号余数。

```rust
fn r#mod (& self , other : & Self) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

other: &Decimal；Rust 返回 Result<Decimal, DecimalError>。余数与被除数同号，小数位数为两个输入中较大者；不是非负的欧几里得取模。零除数返回 DivideByZero。输入仅被借用。mod 是 Rust 关键字，因此使用 r#mod；JS 使用 .mod(other)。两种名称调用相同核心行为。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-r-mod" locale="zh-Hans" />

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
