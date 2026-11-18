---
locale: zh-Hans
source_revision: aad24e4a42d458d995751545010904db4f3e9453abb0fa3c66514c8c59b55494
api_id: Decimal::new_from_int
---
# Decimal::new_from_int

从平台原生有符号整数创建已初始化的十进制数。

```rust
fn new_from_int (value : isize) -> Self
```

## 参数和返回值

`value: isize` 对应整数 JavaScript `number`；结果 scale 为零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int" locale="zh-Hans" />

## 用法注意事项和边界

浏览器目标是 wasm32，接受范围始终为 -2147483648 至 2147483647，即使原生 Rust 运行在 64 位机器上。小数、NaN 和无穷大属于参数错误。

浏览器还应用[资源限制](../resource-policy)。JavaScript 参数在调用 Rust 前进行类型及范围检查。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
