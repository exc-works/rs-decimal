---
locale: ko
source_revision: 561bf3d1118c3ce8a6c2b94160186e43d5ce2153ae6df6a545c57b9a2538ed41
translation_provenance: en-US-korean-reviewed
api_id: Decimal::modulo
---
# Decimal::modulo

0 방향으로 버리는 나눗셈의 부호 있는 나머지를 반환합니다.

```rust
fn modulo (& self , other : & Self) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. Rust는 Result<Decimal, DecimalError>를 반환합니다. 나머지는 피제수의 부호와 두 피연산자 중 큰 소수 자릿수를 가지며 음수가 아닌 유클리드 나머지가 아닙니다. 제수가 0이면 DivideByZero입니다. 입력은 빌려 사용합니다. 몫과 나머지를 함께 얻으려면 quo_rem을 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-modulo" locale="ko" />

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
