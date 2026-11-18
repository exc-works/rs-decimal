---
locale: ko
source_revision: 4f9cc4e1f27490d354a3bbc2ca44814e0fd0d846ebf81964d518e23cc81067c0
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

## 브라우저 제한

위의 소유권 규칙을 따르세요. 이동 후 접근은 moved 오류이며 free는 멱등적입니다. 타입과 별칭 검사는 이동 전에 수행하지만 Rust 오류가 소비한 수신 객체를 복구하지는 않습니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개입니다. 거듭제곱은 중간값을, 자릿수 맞춤은 계수 증가를 검사합니다. 보수적인 검사로 최종 결과가 작아도 거부될 수 있습니다.

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
