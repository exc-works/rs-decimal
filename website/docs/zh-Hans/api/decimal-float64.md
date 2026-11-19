---
locale: zh-Hans
source_revision: 336fb83395651877f8e8be84274d454b59233c0e0829f7aed38b4d6aa19fe60d
api_id: Decimal::float64
---
# Decimal::float64

直接转换为 IEEE binary64 并报告精确性。

```rust
fn float64 (& self) -> (f64 , bool)
```

## 参数、返回值及常规用法

无参数；Rust 返回 (f64, bool)，映射为 [number, exact]。正中间值取偶数。标志表示十进制数被精确表示，而非仅表示转换成功：0.1 不精确。溢出返回带符号无穷和 false；下溢可返回带符号零。示例记录 IEEE 位模式，避免 JSON 隐藏特殊值。Playground print 也会为 NaN、无穷和负零标注传输时的 binary64 位模式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-float64" locale="zh-Hans" />

## 相关方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
