---
locale: zh-Hant
source_revision: fc8796bf45611497be2998723d098452a9849a3946a05ba9bf67b1cf99b43481
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::log10_with_prec
---
# Decimal::log10_with_prec

計算以 10 為底的對數。prec: 非負 isize，以整數 number 表示，指定小數位數；負精度在定義域檢查前回傳 InvalidPrecision。輸入必須為正；零和負數回傳 InvalidLog。底數的精確冪具有整數對數，並保留輸出小數位數。核心使用區間包圍，僅在捨入已驗證時回傳結果。使用 HalfEven，沒有 mode 參數；核心預算內無法驗證時回傳 NoConvergence，不回傳未經驗證的近似值。輸入保持不變，成功結果已初始化。

```rust
fn log10_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 常規範例

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-log10-with-prec" locale="zh-Hant" />

## 相關方法

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
