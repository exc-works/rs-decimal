---
locale: zh-Hans
source_revision: e8f2b09a741088cc99921865f54ef434d0d70ac376d4e29b7efc3c00a51c3956
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

## 用法注意事项和边界

不要先用 JavaScript number 丢失精度后再转 bigint；应直接使用以 `n` 结尾的整数字面量或精确整数文本。Playground 最多接受 4096 位输入数字；原生库不受此 Playground 策略限制。

浏览器还应用[资源限制](../resource-policy)。JavaScript 参数在调用 Rust 前进行类型及范围检查。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
