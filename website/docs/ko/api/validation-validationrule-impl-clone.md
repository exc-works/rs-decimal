---
locale: ko
source_revision: b55dba0922642cda4e225685c47d97a54da01243942c47cb892b8c9136202f1a
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationRule::impl Clone"
---
# validation::ValidationRule::impl Clone

Rust Clone 구현을 사용합니다.

```rust
impl Clone for validation::ValidationRule
```

## 매개변수, 반환값, 일반적인 사용법

Clone은 전체 규칙, 템플릿 맵 또는 실패 필드를 복사합니다. 복사본은 독립적으로 소유됩니다. 네이티브 Rust 트레이트 문법이며 브라우저 메서드가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-impl-clone" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
