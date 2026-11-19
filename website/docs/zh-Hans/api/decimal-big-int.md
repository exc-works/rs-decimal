---
locale: zh-Hans
source_revision: dcbc410f5dabf4a62b4cb667b765d034201be05823aeeb413148caad325fb23f
api_id: Decimal::big_int
---
# Decimal::big_int

复制精确的未缩放整数系数。

```rust
fn big_int (& self) -> BigInt
```

## 参数、返回值及注意事项

Rust 返回独立的 BigInt，bridge 返回 bigint。它包含末尾零，不是截断小数的整数转换。示例为 JSON 输出把 bigint 转成文本。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
