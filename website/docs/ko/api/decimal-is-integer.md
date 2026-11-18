---
locale: ko
source_revision: 6f40656db82d99e820bf25f176794a430859d18b708e5dfb0e4c5ad3ab2117e5
translation_provenance: en-US-korean-reviewed
api_id: Decimal::is_integer
---
# Decimal::is_integer

수치에 소수 부분이 없는지, 즉 정수인지 확인합니다.

```rust
fn is_integer (& self) -> bool
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. bool/boolean을 반환합니다. 소수 자릿수가 0인지가 아니라 수치를 검사하므로 12.00도 정수입니다. 기본 0도 정수이며 소수 부분이 없습니다. 이 조회는 입력을 초기화하거나 변경하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="ko" />

## 브라우저 제한

입력은 빌려 사용하며 성공이나 일반 오류 후에도 사용할 수 있습니다. 반환된 Decimal은 독립적인 데이터를 소유하고 계산 결과는 초기화됩니다. 브라우저는 코어 호출 전에 [리소스 정책](../resource-policy)으로 자릿수 맞춤, 중간 곱, 나눗셈 스케일 조정, 출력 정밀도를 검사합니다. 보수적인 사전 검사로 최종 결과가 작은 계산도 거부될 수 있습니다. 리소스 오류와 수치 라이브러리 오류는 구분하며 패닉은 실행을 끝냅니다.

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
