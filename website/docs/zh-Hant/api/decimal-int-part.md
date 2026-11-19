---
locale: zh-Hant
source_revision: b7ff1544694c51babb7af7cb99735ecc6687d86e9cd9a108af321bed93252beb
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

向零截斷，提取整數部分。

```rust
fn int_part (& self) -> BigInt
```

## 參數、回傳值及常規用法

無參數，Rust 回傳 BigInt，bridge 回傳 bigint，不經過浮點轉換。-12.75 得到 -12，而非 floor(-12.75) 的 -13。預設零得到零。範例將整數序列化為字串，以在 JSON 中保留任意精度。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="zh-Hant" />

## 相關方法

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
