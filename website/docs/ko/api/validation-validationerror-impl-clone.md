---
locale: ko
source_revision: 262ffddf12b7585e3d9a91337def5d4995343306864bd291c3fadc2686ec5701
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationError::impl Clone"
---
# validation::ValidationError::impl Clone

Rust Clone 구현을 사용합니다.

```rust
impl Clone for validation::ValidationError
```

## 매개변수, 반환값, 일반적인 사용법

Clone은 전체 규칙, 템플릿 맵 또는 실패 필드를 복사합니다. 복사본은 독립적으로 소유됩니다. 네이티브 Rust 트레이트 문법이며 브라우저 메서드가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-clone" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
