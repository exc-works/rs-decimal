---
locale: zh-Hans
source_revision: 075546f79884d636c4e552add1629511a5f2dca740186a5700262f084c5bd8a5
api_id: hundred
---
# hundred

构造值为一百的 Decimal 整数。

```rust
fn hundred () -> Decimal
```

## 参数、返回值及注意事项

无参数，返回新的已初始化 Decimal，小数位数为零。它是数值构造函数，不是共享的可变常量。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-hundred" locale="zh-Hans" />

## 边界

bridge 保留 Rust 方法名及参数顺序。自由函数在 Playground 中可直接调用。借用的输入不会被消耗或改变；bridge 拒绝失效或其他运行实例的对象。全部操作遵守 [资源限制](../resource-policy)。

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
