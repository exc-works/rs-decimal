---
locale: ko
source_revision: adde9dcc9ce010498ad91d2410621453cfaa363aae0ee4c3d4598482bb6eead0
translation_provenance: en-US-korean-reviewed
api_id: Decimal::mul
---
# Decimal::mul

지정한 출력 정밀도로 소수를 곱합니다.

```rust
fn mul (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 결과의 소수 자릿수는 두 피연산자 중 큰 값이며 합계가 아닙니다. 곱의 모든 숫자를 유지하려면 mul_exact를 사용하세요. 명시적 mode가 반올림을 제어하며 Unnecessary에서 0이 아닌 숫자를 버려야 하면 RoundUnnecessary를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
