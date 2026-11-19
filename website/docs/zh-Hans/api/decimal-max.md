---
locale: zh-Hans
source_revision: c7a338ce433accfeb61051f993009577dfae7589a629a17bae6043234f27ca38
api_id: Decimal::max
---
# Decimal::max

复制两个数值中较大的一个。

```rust
fn max (& self , other : & Self) -> Self
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回独立的 Decimal。相等时保留第一个操作数的小数位数及初始化状态。Rust 中使用 Decimal::max(&left, &right) 明确选择借用参数的固有方法；Ord::max 会取得所有权。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-max" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
