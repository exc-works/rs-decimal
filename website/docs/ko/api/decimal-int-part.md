---
locale: ko
source_revision: 9743ba1e89051026e818a070b86183de419a65ca646afc51459afa17731968b9
translation_provenance: en-US-korean-reviewed
api_id: Decimal::int_part
---
# Decimal::int_part

0 방향으로 버려 정수 부분을 추출합니다.

```rust
fn int_part (& self) -> BigInt
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. Rust는 BigInt, bridge는 bigint를 반환하며 부동소수점 변환을 하지 않습니다. -12.75의 결과는 -12이며 floor(-12.75) = -13이 아닙니다. 기본 0은 0을 반환합니다. 예제는 JSON에서 임의 정밀도를 유지하도록 정수를 문자열로 직렬화합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-int-part" locale="ko" />

## 브라우저 제한

입력은 빌려 사용하며 성공이나 일반 오류 후에도 사용할 수 있습니다. 반환된 Decimal은 독립적인 데이터를 소유하고 계산 결과는 초기화됩니다. 브라우저는 코어 호출 전에 [리소스 정책](../resource-policy)으로 자릿수 맞춤, 중간 곱, 나눗셈 스케일 조정, 출력 정밀도를 검사합니다. 보수적인 사전 검사로 최종 결과가 작은 계산도 거부될 수 있습니다. 리소스 오류와 수치 라이브러리 오류는 구분하며 패닉은 실행을 끝냅니다.

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
