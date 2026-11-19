---
locale: ko
source_revision: 2d051b010383c0adad95eaeacec22cce35ee5a02d1fddebba48aa1d5a1f4d4ae
translation_provenance: en-US-korean-reviewed
api_id: Decimal::ceil_with_prec
---
# Decimal::ceil_with_prec

지정한 소수 자릿수에서 양의 무한대 방향으로 올립니다.

```rust
fn ceil_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

prec: isize/정수 number. Rust는 Result<Decimal, DecimalError>를, bridge는 Decimal 또는 예외를 반환합니다. 음수 정밀도는 InvalidPrecision입니다. 입력은 빌려 사용하며 변경하지 않습니다. 기본 0이 입력이어도 결과는 초기화됩니다. 정밀도를 높이면 수치를 바꾸지 않고 0을 추가합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil-with-prec" locale="ko" />

## 관련 메서드

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
