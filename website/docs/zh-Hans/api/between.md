---
locale: zh-Hans
source_revision: 536eae84e3e9c7c0c702cbcb339f82c64bf0aafab561d4a3f87590939cb52d8e
api_id: between
---
# between

判断 Decimal 是否位于包含两端点的数值区间内。

```rust
fn between (v : & Decimal , lower : & Decimal , upper : & Decimal) -> bool
```

## 参数、返回值及注意事项

借用 v、lower 和 upper: &Decimal；返回 bool/boolean，判断 lower <= v <= upper。相等按数值判断，忽略表示差异。不会重排边界，lower > upper 时返回 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-between" locale="zh-Hans" />

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
