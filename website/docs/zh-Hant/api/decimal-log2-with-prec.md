---
locale: zh-Hant
source_revision: c212a88589b656fbaa3ad6d1c770f0018ed83f06532037fe1e09556d05cb61ea
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::log2_with_prec
---
# Decimal::log2_with_prec

計算以 2 為底的對數；Rust 回傳 Result<Decimal, DecimalError>，bridge 回傳 Decimal 或拋出結構化錯誤。prec: 非負 isize，以整數 number 表示，指定小數位數；負精度在定義域檢查前回傳 InvalidPrecision。輸入必須為正；零和負數回傳 InvalidLog。底數的精確冪具有整數對數，並保留輸出小數位數。核心使用區間包圍，僅在捨入已驗證時回傳結果。使用 HalfEven，沒有 mode 參數；核心預算內無法驗證時回傳 NoConvergence，不回傳未經驗證的近似值。輸入保持不變，成功結果已初始化。

```rust
fn log2_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 常規範例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log2-with-prec" locale="zh-Hant" />

## 相關方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
