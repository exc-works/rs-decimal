---
locale: zh-Hans
source_revision: f7f692709bde775f34d7f5feb756c59944ff77c763bae1798b2b4306d9770c41
api_id: Decimal::big_int
---
# Decimal::big_int

复制精确的未缩放整数系数。

```rust
fn big_int (& self) -> BigInt
```

## 参数、返回值及注意事项

Rust 返回独立的 BigInt，JavaScript 返回 bigint。它包含末尾零，不是截断小数的整数转换。示例为 JSON 输出把 bigint 转成文本。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
