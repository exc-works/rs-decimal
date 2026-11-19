---
locale: zh-Hant
source_revision: 07fd0624795c1d5a39d29225db8108c9026886d41981fb4ff89e2fb3be01274d
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::exp_with_prec
---
# Decimal::exp_with_prec

計算指數 e^x；Rust 回傳 Result<Decimal, DecimalError>，bridge 回傳 Decimal 或拋出結構化錯誤。prec: 非負 isize，以整數 number 表示，指定小數位數；負精度在定義域檢查前回傳 InvalidPrecision。核心接受所有實數輸入。exp(0) 是已初始化的 1；足夠負的輸入可在指定精度下正確捨入為已初始化的零。核心使用區間包圍，僅在捨入已驗證時回傳結果。使用 HalfEven，沒有 mode 參數；核心預算內無法驗證時回傳 NoConvergence，不回傳未經驗證的近似值。輸入保持不變，成功結果已初始化。

```rust
fn exp_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 常規範例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp-with-prec" locale="zh-Hant" />

## 相關方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
