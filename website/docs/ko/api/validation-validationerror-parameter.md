---
locale: ko
source_revision: 9d8774ce76acc603ce438d5ebe120fafe780695ef55b74ea55b1fbe183940981
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationError.parameter"
---
# validation::ValidationError.parameter

실패한 검증 레코드를 확인합니다.

```rust
parameter: String
```

## 매개변수, 반환값, 일반적인 사용법

공백을 포함하여 원래 규칙 매개변수를 정확하게 유지하는 공개 소유 String/string입니다. {1}로 한 번 치환하며 템플릿을 재귀적으로 해석하지 않습니다. 오류 레코드를 편집해도 규칙이나 소수는 바뀌지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-parameter" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
