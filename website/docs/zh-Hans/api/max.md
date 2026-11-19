---
locale: zh-Hans
source_revision: f49d2dc4c074cf4f52b2a3a197eb1d13f1144942ab8a1df7ea4bf51caaaa350f
api_id: max
---
# max

通过自由函数复制两个 Decimal 中较大的一个。

```rust
fn max (a : & Decimal , b : & Decimal) -> Decimal
```

## 参数、返回值及注意事项

借用 a 和 b: &Decimal，返回独立的 Decimal。数值相等时保留 a 的小数位数及初始化状态，与 Decimal::max 的库语义相同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-max" locale="zh-Hans" />

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
