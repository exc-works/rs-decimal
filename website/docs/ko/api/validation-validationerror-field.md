---
locale: ko
source_revision: 356d7409ac746daa69113ece073c46b80c7b34815cbe6ddfc49fc1c80e686226
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationError.field"
---
# validation::ValidationError.field

실패한 검증 레코드를 확인합니다.

```rust
field: String
```

## 매개변수, 반환값, 일반적인 사용법

호출자가 지정한 필드 이름을 담는 공개 소유 String/string입니다. 속성 조회 경로나 HTML 조각이 아닙니다. 번역 시 중괄호는 그대로 삽입합니다. 오류 레코드를 편집해도 규칙이나 소수는 바뀌지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-field" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
