---
locale: zh-Hans
source_revision: eafc21e4a88e29bea4168f39bfe3b5db899f2a28bccbd47f88733ffdaab27a0c
api_id: Decimal::min
---
# Decimal::min

复制两个数值中较小的一个。

```rust
fn min (& self , other : & Self) -> Self
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回独立的 Decimal。相等时保留第一个操作数的小数位数及初始化状态。Rust 中使用 Decimal::min(&left, &right)，避免选中会取得所有权的 Ord::min trait 方法。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
