---
locale: zh-Hant
source_revision: fbc64b05e8340067582332c66c6f66973b00c5a17412c42b00f322b92b77d915
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::ln_with_prec
---
# Decimal::ln_with_prec

計算自然對數。prec: 非負 isize，以整數 number 表示，指定小數位數；負精度在定義域檢查前回傳 InvalidPrecision。輸入必須為正；零和負數回傳 InvalidLog。ln(1) 精確為零。核心使用區間包圍，僅在捨入已驗證時回傳結果。使用 HalfEven，沒有 mode 參數；核心預算內無法驗證時回傳 NoConvergence，不回傳未經驗證的近似值。輸入保持不變，成功結果已初始化。

```rust
fn ln_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 常規範例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-ln-with-prec" locale="zh-Hant" />

## 相關方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
