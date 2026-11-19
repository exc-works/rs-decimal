---
locale: zh-Hans
source_revision: 41da382d7882f55308ed631f052fb5ac78093f335752392d35d023f744890035
api_id: Decimal::new_from_big_int
---
# Decimal::new_from_big_int

把任意精度整数复制为已初始化、scale 为零的 Decimal，不丢失数字。

```rust
fn new_from_big_int (value : & BigInt) -> Self
```

## 参数和返回值

Rust 借用 `&BigInt`；JavaScript 传入不可变的 `bigint`。返回值的系数独立于来源。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int" locale="zh-Hans" />

## 使用说明

不要先用 JavaScript number 丢失精度后再转 bigint；应直接使用以 `n` 结尾的整数字面量或精确整数文本。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
