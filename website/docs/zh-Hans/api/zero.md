---
locale: zh-Hans
source_revision: 68f642c8ee77ff168dce0e11e2443279259a34decdc0113ce303a8ad64b12c0c
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

## 相关方法

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
