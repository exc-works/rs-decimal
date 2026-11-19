---
locale: zh-Hant
source_revision: 336fb83395651877f8e8be84274d454b59233c0e0829f7aed38b4d6aa19fe60d
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::float64
---
# Decimal::float64

直接轉換為 IEEE binary64 並報告精確性。

```rust
fn float64 (& self) -> (f64 , bool)
```

## 參數、回傳值及常規用法

無參數；Rust 回傳 (f64, bool)，映射為 [number, exact]。正中間值取偶數。旗標表示十進位數被精確表示，而非僅表示轉換成功：0.1 不精確。溢出回傳帶符號無窮和 false；下溢可回傳帶符號零。範例記錄 IEEE 位模式，避免 JSON 隱藏特殊值。Playground print 也會為 NaN、無窮和負零標注傳輸時的 binary64 位模式。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-float64" locale="zh-Hant" />

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
