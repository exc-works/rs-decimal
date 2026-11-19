---
locale: zh-Hans
source_revision: ea763f949f6ea9085a5bdbf656ce6fbf3e8731a7f67b0b4db3858116c77c3d15
api_id: Decimal::lt
---
# Decimal::lt

判断当前数值是否小于另一个数值。

```rust
fn lt (& self , other : & Self) -> bool
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回 bool/boolean。数值相等时返回 false，即使保存的小数位数不同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lt" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
