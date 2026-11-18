---
locale: ko
source_revision: 5fb085491b7b8fca3cac801efd640150df3603d315a453ad87ad49323fbb68e2
translation_provenance: en-US-korean-reviewed
api_id: Decimal::mul_exact
---
# Decimal::mul_exact

소수를 정확하게 곱합니다.

```rust
fn mul_exact (& self , other : & Self) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 결과의 소수 자릿수는 두 피연산자의 자릿수 합계이며 곱의 모든 숫자를 유지합니다. 코어의 소수 자릿수 오버플로는 Overflow를 반환합니다. 반올림 방식 매개변수는 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-exact" locale="ko" />

## 브라우저 제한

입력은 빌려 사용하며 성공이나 일반 오류 후에도 사용할 수 있습니다. 반환된 Decimal은 독립적인 데이터를 소유하고 계산 결과는 초기화됩니다. 브라우저는 코어 호출 전에 [리소스 정책](../resource-policy)으로 자릿수 맞춤, 중간 곱, 나눗셈 스케일 조정, 출력 정밀도를 검사합니다. 보수적인 사전 검사로 최종 결과가 작은 계산도 거부될 수 있습니다. 리소스 오류와 수치 라이브러리 오류는 구분하며 패닉은 실행을 끝냅니다.

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
