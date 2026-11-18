---
locale: ko
source_revision: cf4512111f085d614e6ba264253c0be605dab554b0006351dc9a843e2de414f7
translation_provenance: en-US-korean-reviewed
api_id: Decimal::mul
---
# Decimal::mul

지정한 출력 정밀도로 소수를 곱합니다.

```rust
fn mul (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 결과의 소수 자릿수는 두 피연산자 중 큰 값이며 합계가 아닙니다. 곱의 모든 숫자를 유지하려면 mul_exact를 사용하세요. 명시적 mode가 반올림을 제어하며 Unnecessary에서 0이 아닌 숫자를 버려야 하면 RoundUnnecessary를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul" locale="ko" />

## 브라우저 제한

입력은 빌려 사용하며 성공이나 일반 오류 후에도 사용할 수 있습니다. 반환된 Decimal은 독립적인 데이터를 소유하고 계산 결과는 초기화됩니다. 브라우저는 코어 호출 전에 [리소스 정책](../resource-policy)으로 자릿수 맞춤, 중간 곱, 나눗셈 스케일 조정, 출력 정밀도를 검사합니다. 보수적인 사전 검사로 최종 결과가 작은 계산도 거부될 수 있습니다. 리소스 오류와 수치 라이브러리 오류는 구분하며 패닉은 실행을 끝냅니다.

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
