---
locale: zh-Hant
source_revision: 3d99b7dec25de936ed85161e3eb8cf4565f56b88649e6c879a6468d051308404
translation_provenance: zh-Hans-terminology-reviewed
api_id: Decimal::new_from_big_rat_with_prec
---
# Decimal::new_from_big_rat_with_prec

將有理數捨入到指定小數位數。

```rust
fn new_from_big_rat_with_prec (value : & BigRational , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 參數、回傳值及常規用法

value: &BigRational；prec: 非負 isize；mode: RoundingMode。JavaScript 使用精確的 bigint 分子分母物件。與精確轉換不同，允許 1/3 等無限小數。負精度回傳 InvalidPrecision；Unnecessary 遇到必須捨入時回傳 RoundUnnecessary。零分母在呼叫 Rust 前被 JavaScript 拒絕；零分子回傳 prec 精度的已初始化零。

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-new-from-big-rat-with-prec" locale="zh-Hant" />

## 相關方法

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
