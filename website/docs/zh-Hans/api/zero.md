---
locale: zh-Hans
source_revision: 751751d0b535375fecff33d2510df43e92ffca892248b1be4dba2a34ce1d0f99
api_id: zero
---
# zero

构造已初始化且小数位数为零的零值。

```rust
fn zero () -> Decimal
```

## 参数、返回值及注意事项

无参数，返回新的 Decimal。与 Decimal::default() 不同，结果已初始化；两者的数值仍然相等。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-zero" locale="zh-Hans" />

## 边界

bridge 保留 Rust 方法名及参数顺序。自由函数在 Playground 中可直接调用。借用的输入不会被消耗或改变；bridge 拒绝失效或其他运行实例的对象。全部操作遵守 [资源限制](../resource-policy)。

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
