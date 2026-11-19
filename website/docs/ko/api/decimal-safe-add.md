---
locale: ko
source_revision: 197565e31e34527cbc33f567c65ff6fb5c5304500454cb2da830919e852c5158
translation_provenance: en-US-korean-reviewed
api_id: Decimal::safe_add
---
# Decimal::safe_add

소수를 정확하게 더하고 결과가 음수가 아니어야 함을 요구합니다.

```rust
fn safe_add (& self , other : & Self) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 두 피연산자 중 큰 소수 자릿수로 새 Decimal을 반환합니다. 자릿수 맞춤은 정확하며 숫자를 버리지 않습니다. Rust는 Result를 반환합니다. 음수가 아니어야 하는 것은 각 피연산자가 아니라 결과뿐입니다. 결과가 음수이면 InvalidArgument이며 0은 허용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-add" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
