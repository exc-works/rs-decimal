---
locale: zh-Hans
source_revision: 602dcc7d7a4e6e51eaa040b5b95f43c52285a62967f721872ad5548725d33f50
api_id: Decimal::not_equal
---
# Decimal::not_equal

判断两个 Decimal 的数值是否不同。

```rust
fn not_equal (& self , other : & Self) -> bool
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回 bool/boolean。这是 equal() 的反向判断，不是对系数及小数位数快照的比较。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-not-equal" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
