---
locale: zh-Hans
source_revision: fe9121bf755dd82c88fc1274dc43b09d6173d99c7a22441eb84bc6a73587fa71
api_id: Decimal::new
---
# Decimal::new

创建已初始化的整数，适合有符号 64 位计数或整数金额。

```rust
fn new (value : i64) -> Self
```

## 参数和返回值

`value: i64` 对应 JavaScript `bigint`，结果的 scale 为零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new" locale="zh-Hans" />

## 用法注意事项和边界

使用 `12n`，不能传 JavaScript number。bridge 拒绝超出 i64 的值；小数输入应使用 `new_from_string`。

浏览器还应用[资源限制](../resource-policy)。JavaScript 参数在调用 Rust 前进行类型及范围检查。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
