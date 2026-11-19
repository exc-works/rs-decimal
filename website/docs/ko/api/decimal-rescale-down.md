---
locale: ko
source_revision: e9dbb721c8987afdaedb4f74b013156db52924dc6bfa528e95669ba4396942ff
translation_provenance: en-US-korean-reviewed
api_id: Decimal::rescale_down
---
# Decimal::rescale_down

0 방향으로 버려 소수 자릿수를 바꿉니다.

```rust
fn rescale_down (& self , prec : isize) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

prec: isize/정수 number. Decimal을 반환하며 음수 정밀도는 InvalidPrecision입니다. Down은 음의 무한대가 아니라 0 방향입니다. 정밀도를 높이면 0을 추가합니다. 입력은 변하지 않고 결과는 초기화됩니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale-down" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
