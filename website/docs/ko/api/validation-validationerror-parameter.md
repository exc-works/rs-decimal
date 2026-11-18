---
locale: ko
source_revision: 0e561c16634157ba3586cee74da3c90f15491d42428e944428a8ee980c8ddeb4
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

## 브라우저 제한

validation 기능은 활성화되어 있습니다. 각 문자열은 UTF-8 8192바이트, 규칙/오류 목록과 메시지 맵은 64항목, JSON 전송은 65536바이트까지입니다. 규칙의 소수 매개변수는 [리소스 정책](../resource-policy)을 따릅니다. 번역 전에 치환 텍스트의 보수적인 총합 상한 65536바이트를 검사하므로 실제로는 딱 맞는 결과도 거부할 수 있습니다. 사용자 태그와 필드는 데이터이며 실행 코드가 아닙니다. 검증 실패, 설정 오류, bridge 리소스 오류는 구분합니다.

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
