---
locale: ko
source_revision: 9c252d1ff038e1dac4451e4dd728020f00ea8f88567265b756b2f63f4474b26e
translation_provenance: en-US-korean-reviewed
api_id: Decimal::must_non_negative
---
# Decimal::must_non_negative

음수가 아님을 요구하고 초기화된 복사본을 반환합니다.

```rust
fn must_non_negative (& self) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. 0을 포함한 음수가 아닌 입력에는 Decimal을 반환합니다. 음수 입력은 Rust에서 문자열 페이로드를 가진 패닉을 일으킵니다. Wasm에서는 복구 가능한 DecimalError가 아니라 트랩이 되어 playground 실행을 끝냅니다. 새 실행을 시작하여 Wasm 상태를 새로 만드세요. 이는 수치에 대한 단언이며 절댓값 변환이 아닙니다. 부호를 바꾸려면 abs를 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-must-non-negative" locale="ko" />

## 브라우저 제한

입력은 빌려 사용하며 성공이나 일반 오류 후에도 사용할 수 있습니다. 반환된 Decimal은 독립적인 데이터를 소유하고 계산 결과는 초기화됩니다. 브라우저는 코어 호출 전에 [리소스 정책](../resource-policy)으로 자릿수 맞춤, 중간 곱, 나눗셈 스케일 조정, 출력 정밀도를 검사합니다. 보수적인 사전 검사로 최종 결과가 작은 계산도 거부될 수 있습니다. 리소스 오류와 수치 라이브러리 오류는 구분하며 패닉은 실행을 끝냅니다.

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
