---
locale: zh-Hans
source_revision: 561bf3d1118c3ce8a6c2b94160186e43d5ce2153ae6df6a545c57b9a2538ed41
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

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
