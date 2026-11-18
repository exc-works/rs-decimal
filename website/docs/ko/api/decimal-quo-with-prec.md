---
locale: ko
source_revision: 240d5cd6140ed74d787cde34f7f3b1ea17105d751eeea756af72039c1ea346ce
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

## 브라우저 제한

입력은 빌려 사용하며 성공이나 일반 오류 후에도 사용할 수 있습니다. 반환된 Decimal은 독립적인 데이터를 소유하고 계산 결과는 초기화됩니다. 브라우저는 코어 호출 전에 [리소스 정책](../resource-policy)으로 자릿수 맞춤, 중간 곱, 나눗셈 스케일 조정, 출력 정밀도를 검사합니다. 보수적인 사전 검사로 최종 결과가 작은 계산도 거부될 수 있습니다. 리소스 오류와 수치 라이브러리 오류는 구분하며 패닉은 실행을 끝냅니다.

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
