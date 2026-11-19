---
locale: zh-Hans
source_revision: 7388877c082aab40b6cea1eaae69162f25e5c07d8d3673378f0fdd23fef3e494
api_id: Decimal::scale
---
# Decimal::scale

读取保存的非负小数位数。

```rust
fn scale (& self) -> usize
```

## 参数、返回值及注意事项

返回 usize，在 Playground 精度限制内映射为精确的 number；数值与 precision() 相同。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
