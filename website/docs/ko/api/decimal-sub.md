---
locale: ko
source_revision: 922fb0ccad86b971140048111e8d1bdfebf3b2480d201800dddb60a3c6890884
translation_provenance: en-US-korean-reviewed
api_id: Decimal::sub
---
# Decimal::sub

소수를 정확하게 뺍니다.

```rust
fn sub (& self , other : & Self) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. 두 피연산자 중 큰 소수 자릿수로 새 Decimal을 반환합니다. 자릿수 맞춤은 정확하며 숫자를 버리지 않습니다. Rust 메서드는 실패하지 않지만 브라우저 할당 제한으로 리소스 오류가 발생할 수 있습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub" locale="ko" />

## 브라우저 제한

입력은 빌려 사용하며 성공이나 일반 오류 후에도 사용할 수 있습니다. 반환된 Decimal은 독립적인 데이터를 소유하고 계산 결과는 초기화됩니다. 브라우저는 코어 호출 전에 [리소스 정책](../resource-policy)으로 자릿수 맞춤, 중간 곱, 나눗셈 스케일 조정, 출력 정밀도를 검사합니다. 보수적인 사전 검사로 최종 결과가 작은 계산도 거부될 수 있습니다. 리소스 오류와 수치 라이브러리 오류는 구분하며 패닉은 실행을 끝냅니다.

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
