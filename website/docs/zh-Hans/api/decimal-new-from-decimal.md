---
locale: zh-Hans
source_revision: 37b6ebebec69abdf2809e71eb7e3cad7353a792a069c7217103a51f2eea16c7f
api_id: Decimal::new_from_decimal
---
# Decimal::new_from_decimal

复制现有 Decimal，同时保留其存储表示和初始化状态。

```rust
fn new_from_decimal (value : & Self) -> Self
```

## 参数和返回值

借用 `value: &Decimal`，返回独立的 Decimal；输入仍可继续使用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-decimal" locale="zh-Hans" />

## 用法注意事项和边界

复制 `Decimal::default()` 会保留未初始化状态，不会变成已初始化的零。JavaScript 必须传入同一次运行中仍有效的 Decimal；此构造器不消耗它。

浏览器还应用[资源限制](../resource-policy)。JavaScript 参数在调用 Rust 前进行类型及范围检查。

## 相关方法

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
