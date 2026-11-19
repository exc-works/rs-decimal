---
locale: zh-Hans
source_revision: 10c5c67f0eca13950935316848dbaf7b4f6fbe37a761aaabb0a34ec3fe68d1c2
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

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
