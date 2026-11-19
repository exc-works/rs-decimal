---
locale: ko
source_revision: b2363400afff1e4c740938a16816a148faaff0faef10b304c45a776ef2dcc1ee
translation_provenance: en-US-korean-reviewed
api_id: Decimal::mul_with_prec
---
# Decimal::mul_with_prec

지정한 출력 정밀도로 소수를 곱합니다.

```rust
fn mul_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 결과의 소수 자릿수는 prec이며 음수가 아니어야 합니다. 음수 prec는 InvalidPrecision을 반환합니다. 명시적 mode가 반올림을 제어하며 Unnecessary에서 0이 아닌 숫자를 버려야 하면 RoundUnnecessary를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-with-prec" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
