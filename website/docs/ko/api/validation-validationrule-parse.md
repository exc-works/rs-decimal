---
locale: ko
source_revision: 906bb0bd7555c68ed2da4c7be205d647a1ca94b724e2e70f5d1160d968357278
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationRule::parse"
---
# validation::ValidationRule::parse

정확한 소수 규칙을 평가합니다.

```rust
fn parse (tag : & str , parameter : & str) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

tag: &str/string, parameter: &str/string. `Result<ValidationRule, DecimalError>`/규칙 핸들을 반환합니다. decimal_gt 같은 전체 태그를 사용하세요. 비교 매개변수는 소수 텍스트, between은 lower~upper, max_precision은 음수가 아닌 isize를 받습니다. 알 수 없는 태그, 잘못된 매개변수, 뒤집힌 범위는 InvalidArgument입니다. wasm32의 isize는 부호 있는 32비트입니다. 매개변수 없는 규칙도 매개변수 텍스트를 유지하지만 해석하지 않습니다. 규칙은 부동소수점 변환 없이 정확한 Decimal 값을 비교합니다. decimal_required는 초기화를 검사하므로 초기화된 0은 통과합니다. eq/ne/gt/gte/lt/lte는 미초기화 0도 수치로 비교합니다. between은 양 끝을 포함하는 lower~upper를 사용하며 초기화도 요구합니다. positive/negative/nonzero도 초기화를 요구합니다. max_precision은 유지된 후행 0을 포함한 저장 소수 자릿수를 검사하며 required를 암묵적으로 요구하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-parse" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
