---
locale: ko
source_revision: d6f697a3d962962b5c45eb345607fbc6629bac0f2d7f0f5c403da5649a7d1d4a
translation_provenance: en-US-korean-reviewed
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

지정한 소수 자릿수에서 가장 가까운 값으로 반올림하며 중간이면 짝수 쪽을 택합니다.

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## 매개변수

`prec: isize` — 유지할 소수 자릿수(정수).

## 반환값

기본 영 값을 입력해도 초기화된 Decimal을 반환합니다. 입력을 빌리며 정밀도를 높이면 끝에 0을 붙입니다.

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). 다른 반올림 모드는 [`rescale`](./decimal-rescale) 호출로 지정합니다.

## 오류

음수 정밀도가 반환하는 오류는 `InvalidPrecision`. JavaScript는 해당 오류를 던집니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
