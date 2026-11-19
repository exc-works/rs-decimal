---
locale: ko
source_revision: b834c8dbe54adc1326f4715bae0b5a7e4a551bc4817e951238d3a76cb1960aa5
translation_provenance: en-US-korean-reviewed
api_id: Decimal::safe_sub
---
# Decimal::safe_sub

소수를 정확하게 빼고 결과가 음수가 아니어야 함을 요구합니다.

```rust
fn safe_sub (& self , other : & Self) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 두 피연산자 중 큰 소수 자릿수로 새 Decimal을 반환합니다. 자릿수 맞춤은 정확하며 숫자를 버리지 않습니다. Rust는 Result를 반환합니다. 음수가 아니어야 하는 것은 각 피연산자가 아니라 결과뿐입니다. 결과가 음수이면 InvalidArgument이며 0은 허용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-sub" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
