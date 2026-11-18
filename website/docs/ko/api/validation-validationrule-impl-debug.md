---
locale: ko
source_revision: 3ccd35e370bf1f1ede8c557955aa9396976fa4ff57cb089c5b7cc2f5b8ec4b48
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationRule::impl Debug"
---
# validation::ValidationRule::impl Debug

Rust Debug 구현을 사용합니다.

```rust
impl Debug for validation::ValidationRule
```

## 매개변수, 반환값, 일반적인 사용법

Debug는 진단 텍스트이며 안정적인 검증 직렬화 형식이 아닙니다. 네이티브 Rust 트레이트 문법이며 브라우저 메서드가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-impl-debug" locale="ko" />

## 브라우저 제한

validation 기능은 활성화되어 있습니다. 각 문자열은 UTF-8 8192바이트, 규칙/오류 목록과 메시지 맵은 64항목, JSON 전송은 65536바이트까지입니다. 규칙의 소수 매개변수는 [리소스 정책](../resource-policy)을 따릅니다. 번역 전에 치환 텍스트의 보수적인 총합 상한 65536바이트를 검사하므로 실제로는 딱 맞는 결과도 거부할 수 있습니다. 사용자 태그와 필드는 데이터이며 실행 코드가 아닙니다. 검증 실패, 설정 오류, bridge 리소스 오류는 구분합니다.

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
