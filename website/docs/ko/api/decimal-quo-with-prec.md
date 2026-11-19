---
locale: ko
source_revision: eb9bcc338f948ddd5ec14bfd00bb5bb38695bb5e5eba9b781fe855d89569b0a0
translation_provenance: en-US-korean-reviewed
api_id: Decimal::quo_with_prec
---
# Decimal::quo_with_prec

명시적인 정밀도 또는 입력에서 정한 정밀도로 소수를 나눕니다.

```rust
fn quo_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. Rust는 Result<Decimal, DecimalError>를 반환합니다. prec는 음수가 아닌 소수 자릿수이며 음수 정밀도는 제수 검사 전에 InvalidPrecision을 반환합니다. 제수가 0이면 DivideByZero입니다. mode가 반올림을 제어하며 Unnecessary에서 부정확한 결과가 필요하면 RoundUnnecessary를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-with-prec" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
