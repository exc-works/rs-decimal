---
locale: zh-Hans
source_revision: 19e13f576512aa6ef6c12c54900338d87cb485da07e25d15d575e4c5353b491a
api_id: Decimal::is_not_zero
---
# Decimal::is_not_zero

判断数值是否非零。

```rust
fn is_not_zero (& self) -> bool
```

## 参数、返回值及注意事项

返回 bool/boolean，与 is_zero() 相反；未初始化的默认值返回 false。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-not-zero" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
