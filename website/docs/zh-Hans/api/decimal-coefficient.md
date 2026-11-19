---
locale: zh-Hans
source_revision: 8e571c100355b6f464b7992d5d9d515f9f9923a4538f1e43b8cc48af4b41a4c7
api_id: Decimal::coefficient
---
# Decimal::coefficient

查看精确的未缩放整数系数。

```rust
fn coefficient (& self) -> & BigInt
```

## 参数、返回值及注意事项

Rust 借用 &BigInt；JavaScript 返回不可变的 bigint 快照，并非实时引用。系数包含末尾零，不是小数的整数部分。示例把 bigint 转成文本，以便 JSON 保留全部数字。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
