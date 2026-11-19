---
locale: ko
source_revision: 0b44060089288355d3eba23edfcc95ed4174aef13953ff7dc6b74df01f566b11
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationError.tag"
---
# validation::ValidationError.tag

실패한 검증 레코드를 확인합니다.

```rust
tag: & 'static str
```

## 매개변수, 반환값, 일반적인 사용법

실패한 규칙을 식별하는 공개 Rust 정적 문자열입니다. 브라우저에서는 소유하는 문자열 속성이며 알 수 없는 태그를 번역하면 코어 기본 템플릿 "{0} is invalid"를 사용합니다. 오류 레코드를 편집해도 규칙이나 소수는 바뀌지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-tag" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
