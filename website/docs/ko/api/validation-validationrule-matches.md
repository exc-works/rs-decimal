---
locale: ko
source_revision: 816a5e8449654efb901339ad5516b3fdb94be2af25e3c709a61823154b83be91
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

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
