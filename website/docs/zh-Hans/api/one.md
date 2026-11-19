---
locale: zh-Hans
source_revision: 46f70cc4972a4c4ca833f0191402c07a03bbeeb19c10706e521c3bfe03276dbf
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

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
