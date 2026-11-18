---
locale: ko
source_revision: 788e3facaf7bb31d022529aaea08540d91c8a4001537b0085ab4e604ef0e3ae8
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationRule::matches"
---
# validation::ValidationRule::matches

정확한 소수 규칙을 평가합니다.

```rust
fn matches (& self , value : & Decimal) -> bool
```

## 매개변수, 반환값, 일반적인 사용법

value: &Decimal/Decimal. bool/boolean을 반환하고 규칙과 값을 모두 유지합니다. 규칙은 부동소수점 변환 없이 정확한 Decimal 값을 비교합니다. decimal_required는 초기화를 검사하므로 초기화된 0은 통과합니다. eq/ne/gt/gte/lt/lte는 미초기화 0도 수치로 비교합니다. between은 양 끝을 포함하는 lower~upper를 사용하며 초기화도 요구합니다. positive/negative/nonzero도 초기화를 요구합니다. max_precision은 유지된 후행 0을 포함한 저장 소수 자릿수를 검사하며 required를 암묵적으로 요구하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-matches" locale="ko" />

## 브라우저 제한

validation 기능은 활성화되어 있습니다. 각 문자열은 UTF-8 8192바이트, 규칙/오류 목록과 메시지 맵은 64항목, JSON 전송은 65536바이트까지입니다. 규칙의 소수 매개변수는 [리소스 정책](../resource-policy)을 따릅니다. 번역 전에 치환 텍스트의 보수적인 총합 상한 65536바이트를 검사하므로 실제로는 딱 맞는 결과도 거부할 수 있습니다. 사용자 태그와 필드는 데이터이며 실행 코드가 아닙니다. 검증 실패, 설정 오류, bridge 리소스 오류는 구분합니다.

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
