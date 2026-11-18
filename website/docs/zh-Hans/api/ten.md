---
locale: zh-Hans
source_revision: df5bdc8fba1feff0933958fdceab2cd0bf8d1a541a0f2e47f17bffe6a6338d47
api_id: ten
---
# ten

构造值为十的 Decimal 整数。

```rust
fn ten () -> Decimal
```

## 参数、返回值及注意事项

无参数，返回新的已初始化 Decimal，小数位数为零。需要小数位数时使用 rescale。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-ten" locale="zh-Hans" />

## 边界

bridge 保留 Rust 方法名及参数顺序。自由函数在 Playground 中可直接调用。借用的输入不会被消耗或改变；bridge 拒绝失效或其他运行实例的对象。全部操作遵守 [资源限制](../resource-policy)。

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
