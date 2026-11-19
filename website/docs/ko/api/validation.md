---
locale: ko
source_revision: 668ba6d01bb4e8f8fc531b677bb3554fa942e3fa2858410dfc728efffb55ae7f
translation_provenance: en-US-korean-reviewed
api_id: "validation"
---
# validation

규칙 실패를 순서대로 모아 번역합니다.

```rust
pub mod validation
```

## 매개변수, 반환값, 일반적인 사용법

Rust에서는 브라우저 빌드에 켜져 있는 validation 기능이 필요합니다. 모듈은 프레임워크나 전역 레지스트리 없이 정확한 규칙 평가와 소유 메시지 템플릿을 묶습니다. 브라우저에서는 validation 네임스페이스를 사용합니다. 필드 이름은 호출자의 텍스트이며 번역 시 그대로 삽입합니다. 누락된 입력을 실패로 처리하려면 required를 명시하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
