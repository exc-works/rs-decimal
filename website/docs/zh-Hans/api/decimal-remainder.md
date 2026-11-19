---
locale: zh-Hans
source_revision: 64d94a480e185cbb38c975ef60712d5ae4ac373961962b675392db6ef9375f09
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

## 相关方法

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
