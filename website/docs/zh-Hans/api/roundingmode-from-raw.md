---
locale: zh-Hans
source_revision: e9709f45197c4a553d9eef583ec3ace47a8ab16090d52890478f7fffb689b911
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

验证整数舍入模式编码。

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## 参数、返回值及常规用法

value: i32/整数 number。0–6 返回对应模式，其他 i32 值返回 InvalidArgument。浏览器小数、非有限或超出 i32 范围的值会在 Rust 调用前被参数检查拒绝。bridge 调用 Rust 验证器并返回数字模式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="zh-Hans" />

## 相关方法

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
