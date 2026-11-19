---
locale: ko
source_revision: d9bfbd2d29924d270b97f6a4251656ca421cb89e51e3bdcdc1ecdf2cdefab10f
translation_provenance: en-US-korean-reviewed
api_id: Decimal::truncate_with_prec
---
# Decimal::truncate_with_prec

지정한 소수 자릿수에서 0 방향으로 버립니다.

```rust
fn truncate_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

prec: isize/정수 number. 음수 정밀도는 InvalidPrecision입니다. 음수에서는 0 방향 버림과 floor가 다릅니다. -1.5를 버리면 -1이지만 floor는 -2입니다. 입력은 빌려 사용하며 변경하지 않습니다. 기본 0이 입력이어도 결과는 초기화됩니다. 정밀도를 높이면 수치를 바꾸지 않고 0을 추가합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate-with-prec" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
