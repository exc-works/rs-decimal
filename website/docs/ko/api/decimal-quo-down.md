---
locale: ko
source_revision: 24cee5102dc473cb8fed8902e9829dd352863649ca89859b6c19bb3f001aa710
translation_provenance: en-US-korean-reviewed
api_id: Decimal::quo_down
---
# Decimal::quo_down

명시적인 정밀도 또는 입력에서 정한 정밀도로 소수를 나눕니다.

```rust
fn quo_down (& self , other : & Self) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. Rust는 Result<Decimal, DecimalError>를 반환합니다. 출력 소수 자릿수는 두 피연산자 중 큰 값입니다. 따라서 두 정수 입력은 정수 결과를 만듭니다. 1 / 3은 0입니다. 소수 결과에는 quo_with_prec를 사용하세요. 제수가 0이면 DivideByZero를 반환합니다. Down은 음의 무한대가 아니라 0 방향으로 버립니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-down" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
