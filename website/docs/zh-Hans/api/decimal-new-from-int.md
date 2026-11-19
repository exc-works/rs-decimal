---
locale: zh-Hans
source_revision: dd3a6777ec0d2daf335730b8ca67dd44bd500850bf5f5a894aa8a18b1d5c40f9
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

## 使用说明

浏览器目标是 wasm32，接受范围始终为 -2147483648 至 2147483647，即使原生 Rust 运行在 64 位机器上。小数、NaN 和无穷大属于参数错误。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
