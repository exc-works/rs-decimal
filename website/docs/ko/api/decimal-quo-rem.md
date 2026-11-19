---
locale: ko
source_revision: 10c5c67f0eca13950935316848dbaf7b4f6fbe37a761aaabb0a34ec3fe68d1c2
translation_provenance: en-US-korean-reviewed
api_id: Decimal::quo_rem
---
# Decimal::quo_rem

정수 몫과 부호 있는 Decimal 나머지를 반환합니다.

```rust
fn quo_rem (& self , other : & Self) -> Result < (Self , Self) , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. Rust는 Result<(Decimal, Decimal), DecimalError>를 반환하며 [quotient, remainder]에 대응합니다. 몫은 0 방향으로 버린 소수 자릿수 0의 값입니다. 나머지는 피제수의 부호와 두 피연산자 중 큰 소수 자릿수를 가지며 self = quotient * other + remainder를 만족합니다. 각 결과는 독립적으로 초기화된 래퍼이고 입력도 계속 사용할 수 있습니다. 제수가 0이면 자릿수 맞춤 전에 DivideByZero를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-quo-rem" locale="ko" />

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
