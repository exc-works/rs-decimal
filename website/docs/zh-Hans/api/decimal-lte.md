---
locale: zh-Hans
source_revision: 633e4a423f3366a4dc502e8d35d4ffa6a3d6f7dd5647951875fd3e31e57bf2c5
api_id: Decimal::lte
---
# Decimal::lte

判断当前数值是否小于或等于另一个数值。

```rust
fn lte (& self , other : & Self) -> bool
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回 bool/boolean。相等按数值判断，与小数位数及初始化状态无关。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lte" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
