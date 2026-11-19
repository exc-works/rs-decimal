---
locale: zh-Hans
source_revision: 9f58b0f5c6b478fabd100e00297f6dfcdae4ff2a32ead701d8b208c279651c42
api_id: min
---
# min

通过自由函数复制两个 Decimal 中较小的一个。

```rust
fn min (a : & Decimal , b : & Decimal) -> Decimal
```

## 参数、返回值及注意事项

借用 a 和 b: &Decimal，返回独立的 Decimal。数值相等时保留 a 的小数位数及初始化状态，与 Decimal::min 的库语义相同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-min" locale="zh-Hans" />

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
