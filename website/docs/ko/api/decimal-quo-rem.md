---
locale: ko
source_revision: ab458b72e36df078d37dd6de4be21a55e10d1e6c955c47b4abcbe87cbbdd8e23
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

## 브라우저 제한

위의 소유권 규칙을 따르세요. 이동 후 접근은 moved 오류이며 free는 멱등적입니다. 타입과 별칭 검사는 이동 전에 수행하지만 Rust 오류가 소비한 수신 객체를 복구하지는 않습니다. [리소스 정책](../resource-policy)은 라이브러리 오류와 별개입니다. 거듭제곱은 중간값을, 자릿수 맞춤은 계수 증가를 검사합니다. 보수적인 검사로 최종 결과가 작아도 거부될 수 있습니다.

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
