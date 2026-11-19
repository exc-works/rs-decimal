---
locale: ko
source_revision: 8f612ca2b723227e5991f08e31b92d45cb4ea058472e86db2c02e8a1c4cc372e
translation_provenance: en-US-korean-reviewed
api_id: Decimal::mul_down
---
# Decimal::mul_down

지정한 출력 정밀도로 소수를 곱합니다.

```rust
fn mul_down (& self , other : & Self) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 결과의 소수 자릿수는 두 피연산자 중 큰 값이며 합계가 아닙니다. 곱의 모든 숫자를 유지하려면 mul_exact를 사용하세요. Down은 음수 결과도 0 방향으로 버립니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-down" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
