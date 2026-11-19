---
locale: ko
source_revision: cfe29d83921c9d9fef3ddeed3842336ccc12799d1b0ed231fac999e4d87de7f7
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationError::impl Eq"
---
# validation::ValidationError::impl Eq

Rust Eq 구현을 사용합니다.

```rust
impl Eq for validation::ValidationError
```

## 매개변수, 반환값, 일반적인 사용법

ValidationError 동등성은 번역 메시지만이 아니라 field, tag, 원래 parameter를 함께 비교합니다. 네이티브 Rust 트레이트 문법이며 브라우저 메서드가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-eq" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
