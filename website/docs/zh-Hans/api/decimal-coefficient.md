---
locale: zh-Hans
source_revision: 9626ec029554d531f95257728b45431467328baf3b8968c06412f5cc58c689ec
api_id: Decimal::coefficient
---
# Decimal::coefficient

查看精确的未缩放整数系数。

```rust
fn coefficient (& self) -> & BigInt
```

## 参数、返回值及注意事项

Rust 借用 &BigInt；bridge 返回不可变的 bigint 快照，并非实时引用。系数包含末尾零，不是小数的整数部分。示例把 bigint 转成文本，以便 JSON 保留全部数字。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
