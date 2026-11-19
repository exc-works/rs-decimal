---
locale: ko
source_revision: 521743c60e4285630a1e202d92aebd0d0b9f4764a73c56a223f9ec0c5662a6d6
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationError"
---
# validation::ValidationError

실패한 검증 레코드를 확인합니다.

```rust
pub struct ValidationError
```

## 매개변수, 반환값, 일반적인 사용법

공개 field, tag, parameter 속성을 가진 실패 레코드입니다. ValidationRule.validate는 실패 시 이를 반환하고 validate_decimal은 목록을 반환합니다. JavaScript는 번역에 적합한 편집 가능한 필드 레코드를 공개합니다. DecimalError 종류나 예외 메시지가 아닙니다. 오류 레코드를 편집해도 규칙이나 소수는 바뀌지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
