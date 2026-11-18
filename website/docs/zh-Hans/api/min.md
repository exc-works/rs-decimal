---
locale: zh-Hans
source_revision: 9a05736a78d12045af2704500028fdc4fdec0490c797ca6d8ba9130b66fbc153
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

## 边界

bridge 保留 Rust 方法名及参数顺序。自由函数在 Playground 中可直接调用。借用的输入不会被消耗或改变；bridge 拒绝失效或其他运行实例的对象。全部操作遵守 [资源限制](../resource-policy)。

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
