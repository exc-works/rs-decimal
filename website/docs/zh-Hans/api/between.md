---
locale: zh-Hans
source_revision: 4488da41f235921047e8f85b8a68b47bc497bf199529326773333d8f22d38585
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

## 边界

bridge 保留 Rust 方法名及参数顺序。自由函数在 Playground 中可直接调用。借用的输入不会被消耗或改变；bridge 拒绝失效或其他运行实例的对象。全部操作遵守 [资源限制](../resource-policy)。

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
