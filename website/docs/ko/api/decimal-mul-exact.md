---
locale: ko
source_revision: 74fe800675809309f5a6802d09df41ce125ec348c7dc823b0a9a763856b0bb52
translation_provenance: en-US-korean-reviewed
api_id: Decimal::mul_exact
---
# Decimal::mul_exact

소수를 정확하게 곱합니다.

```rust
fn mul_exact (& self , other : & Self) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 결과의 소수 자릿수는 두 피연산자의 자릿수 합계이며 곱의 모든 숫자를 유지합니다. 코어의 소수 자릿수 오버플로는 Overflow를 반환합니다. 반올림 방식 매개변수는 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-exact" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
