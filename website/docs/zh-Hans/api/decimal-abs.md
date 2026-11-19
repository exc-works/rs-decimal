---
locale: zh-Hans
source_revision: c71c6b4ce225ea223944d584a5ec03b1adfe1dbd2adeb04f6beb38800f1e252e
api_id: Decimal::abs
---
# Decimal::abs

创建保存绝对值的独立 Decimal。

```rust
fn abs (& self) -> Self
```

## 参数、返回值及注意事项

返回 Decimal，保留小数位数及末尾零。即使输入为默认值，结果也会初始化；输入保持不变。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="zh-Hans" />

## 相关方法

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
