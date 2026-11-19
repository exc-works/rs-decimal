---
locale: zh-Hans
source_revision: c7bf1c667592f8239154dc21841d27e0f4e94230da52fdab553a96aeaa5e48bf
api_id: Decimal::float32
---
# Decimal::float32

直接转换为 IEEE binary32 并报告精确性。

```rust
fn float32 (& self) -> (f32 , bool)
```

## 参数、返回值及常规用法

无参数；Rust 返回 (f32, bool)，映射为 [number, exact]。正中间值取偶数。标志表示十进制数被精确表示，而非仅表示转换成功：0.1 不精确。溢出返回带符号无穷和 false；下溢可返回带符号零。直接舍入到 binary32，不先舍入到 binary64。示例记录 IEEE 位模式，避免 JSON 隐藏特殊值。Playground print 也会为 NaN、无穷和负零标注传输时的 binary64 位模式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-float32" locale="zh-Hans" />

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
