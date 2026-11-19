---
locale: ko
source_revision: 0d6262b2eb7c6955f187e4d63b88d16a39d035f514ca55c738d74b10a8d69203
translation_provenance: en-US-korean-reviewed
api_id: Decimal::quo
---
# Decimal::quo

명시적인 정밀도 또는 입력에서 정한 정밀도로 소수를 나눕니다.

```rust
fn quo (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. Rust는 Result<Decimal, DecimalError>를 반환합니다. 출력 소수 자릿수는 두 피연산자 중 큰 값입니다. 따라서 두 정수 입력은 정수 결과를 만듭니다. 1 / 3은 Down에서는 0, Up에서는 1입니다. 소수 결과에는 quo_with_prec를 사용하세요. 제수가 0이면 DivideByZero를 반환합니다. mode가 반올림을 제어하며 Unnecessary에서 부정확한 결과가 필요하면 RoundUnnecessary를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
