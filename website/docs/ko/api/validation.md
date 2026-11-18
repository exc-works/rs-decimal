---
locale: ko
source_revision: 8a30b5196c371952fcc2508159879f3619eaf7085b839003d31c6721b7f9cf86
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

## 브라우저 제한

validation 기능은 활성화되어 있습니다. 각 문자열은 UTF-8 8192바이트, 규칙/오류 목록과 메시지 맵은 64항목, JSON 전송은 65536바이트까지입니다. 규칙의 소수 매개변수는 [리소스 정책](../resource-policy)을 따릅니다. 번역 전에 치환 텍스트의 보수적인 총합 상한 65536바이트를 검사하므로 실제로는 딱 맞는 결과도 거부할 수 있습니다. 사용자 태그와 필드는 데이터이며 실행 코드가 아닙니다. 검증 실패, 설정 오류, bridge 리소스 오류는 구분합니다.

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
