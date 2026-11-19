---
locale: zh-Hans
source_revision: 94e5c200ef4c48b600cdc1b46dd606d6ca8e9e2741aedbb2bac50f53ac53727e
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

## 使用说明

使用 `12n`，不能传 JavaScript number。JavaScript 拒绝超出 i64 的值；小数输入应使用 `new_from_string`。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
