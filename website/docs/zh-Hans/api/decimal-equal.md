---
locale: zh-Hans
source_revision: 05d4c976c4f23a3286f06a831a12b99e1cda6e2ba4014722196a6ec9aade2d22
api_id: Decimal::equal
---
# Decimal::equal

判断两个 Decimal 的数值是否相同。

```rust
fn equal (& self , other : & Self) -> bool
```

## 参数、返回值及注意事项

借用 other: &Decimal，返回 bool/boolean，忽略小数位数及初始化状态。JavaScript 的 === 比较包装对象身份；数值相等应调用 equal()。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-equal" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
