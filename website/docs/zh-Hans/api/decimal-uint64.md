---
locale: zh-Hans
source_revision: c526a43d69654fb21b49f56d858016f45f88e7f7f6ec8eaa0c13c3e5e939cd03
api_id: Decimal::uint64
---
# Decimal::uint64

转换为固定位宽整数并返回成功标志。

```rust
fn uint64 (& self) -> (u64 , bool)
```

## 参数、返回值及常规用法

无参数；返回 (u64, bool)，映射为 [bigint, boolean]。范围内的整数返回对应整数和 true。小数或溢出返回 (0, false)，不会截断或抛出错误。负数同样返回 (0, false)。不能仅凭 scale 判断是否为整数：12.00 转换成功。应检查标志，区分失败与有效零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-uint64" locale="zh-Hans" />

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
