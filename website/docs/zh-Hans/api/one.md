---
locale: zh-Hans
source_revision: 939b86ba367c0b35cb69b9f2664eb74fafe4279f8b1b81c20315cc4e07b18382
api_id: one
---
# one

构造值为一的 Decimal 整数。

```rust
fn one () -> Decimal
```

## 参数、返回值及注意事项

无参数，返回新的已初始化 Decimal，小数位数为零。修改或释放一次调用的结果不会影响后续调用。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-one" locale="zh-Hans" />

## 边界

bridge 保留 Rust 方法名及参数顺序。自由函数在 Playground 中可直接调用。借用的输入不会被消耗或改变；bridge 拒绝失效或其他运行实例的对象。全部操作遵守 [资源限制](../resource-policy)。

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
