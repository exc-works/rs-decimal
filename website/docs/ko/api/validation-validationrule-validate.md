---
locale: ko
source_revision: 8ffc267a7c40a9915ad1544633aa5847696df2f114f2a6275b83514da10320eb
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationRule::validate"
---
# validation::ValidationRule::validate

하나의 필드를 규칙으로 검증합니다.

```rust
fn validate (& self , field : & str , value : & Decimal) -> Result < () , ValidationError >
```

## 매개변수, 반환값, 일반적인 사용법

field: &str/string, value: &Decimal/Decimal. `Result<(), ValidationError>`를 반환합니다. JavaScript에서 성공은 undefined입니다. 실패하면 category="validation"과 field/tag/parameter 속성을 가진 JS ValidationError를 던지며 DecimalError와 다릅니다. 필드와 원래 매개변수 텍스트는 중괄호와 공백까지 유지합니다. 규칙과 값은 빌립니다. 예외 없이 여러 실패를 모으려면 validate_decimal을 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-validate" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
