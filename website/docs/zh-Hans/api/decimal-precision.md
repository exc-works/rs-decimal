---
locale: zh-Hans
source_revision: 0ad97f33b356cd281b88951f90d8d3c10dff06f728f22f7136b85c34bc769e4e
api_id: Decimal::precision
---
# Decimal::precision

读取数值保存的小数位数。

```rust
fn precision (& self) -> isize
```

## 参数、返回值及注意事项

返回 isize，在浏览器中为整数 number。这是小数位数，不是有效数字位数；末尾零也计入其中。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
