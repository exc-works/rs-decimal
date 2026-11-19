---
locale: ko
source_revision: 4a04c1296f4a9e4d0c1eac8d1707242f3d7b87d7302eac99dc6a590f2c03208e
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationRule"
---
# validation::ValidationRule

정확한 소수 규칙을 평가합니다.

```rust
pub struct ValidationRule
```

## 매개변수, 반환값, 일반적인 사용법

parse(tag, parameter)로 규칙을 만든 뒤 matches나 validate에서 재사용합니다. 규칙 매개변수는 생성 시 한 번 검사합니다. 브라우저 핸들은 한 실행에 속하며 명시적으로 해제할 수 있습니다. 규칙은 부동소수점 변환 없이 정확한 Decimal 값을 비교합니다. decimal_required는 초기화를 검사하므로 초기화된 0은 통과합니다. eq/ne/gt/gte/lt/lte는 미초기화 0도 수치로 비교합니다. between은 양 끝을 포함하는 lower~upper를 사용하며 초기화도 요구합니다. positive/negative/nonzero도 초기화를 요구합니다. max_precision은 유지된 후행 0을 포함한 저장 소수 자릿수를 검사하며 required를 암묵적으로 요구하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
