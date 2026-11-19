---
locale: ko
source_revision: 7f1c2832917badced6e78a9c287520234452b297109e8406a028c3703581c859
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationError::impl PartialEq"
---
# validation::ValidationError::impl PartialEq

Rust PartialEq 구현을 사용합니다.

```rust
impl PartialEq for validation::ValidationError
```

## 매개변수, 반환값, 일반적인 사용법

ValidationError 동등성은 번역 메시지만이 아니라 field, tag, 원래 parameter를 함께 비교합니다. 네이티브 Rust 트레이트 문법이며 브라우저 메서드가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-partialeq" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
