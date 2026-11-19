---
locale: zh-Hans
source_revision: 3a7d890770468c269032ca95ee3365a1f5fdeb70051590bf674c733d1e1658d6
api_id: Decimal::is_zero
---
# Decimal::is_zero

在后续操作前判断数值是否为零。

```rust
fn is_zero (& self) -> bool
```

## 参数、返回值及注意事项

返回 bool/boolean，未初始化的默认值也返回 true。需要区分初始化状态时使用 is_nil()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
