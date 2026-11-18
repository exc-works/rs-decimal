---
locale: ko
source_revision: 377152e0483ddcbebcd02f161fe043c129efdc1f915e3eb70b399c28e3d3906b
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationError"
---
# validation::ValidationError

실패한 검증 레코드를 확인합니다.

```rust
pub struct ValidationError
```

## 매개변수, 반환값, 일반적인 사용법

공개 field, tag, parameter 속성을 가진 실패 레코드입니다. ValidationRule.validate는 실패 시 이를 반환하고 validate_decimal은 목록을 반환합니다. JavaScript는 번역에 적합한 편집 가능한 필드 레코드를 공개합니다. DecimalError 종류나 예외 메시지가 아닙니다. 오류 레코드를 편집해도 규칙이나 소수는 바뀌지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="ko" />

## 브라우저 제한

validation 기능은 활성화되어 있습니다. 각 문자열은 UTF-8 8192바이트, 규칙/오류 목록과 메시지 맵은 64항목, JSON 전송은 65536바이트까지입니다. 규칙의 소수 매개변수는 [리소스 정책](../resource-policy)을 따릅니다. 번역 전에 치환 텍스트의 보수적인 총합 상한 65536바이트를 검사하므로 실제로는 딱 맞는 결과도 거부할 수 있습니다. 사용자 태그와 필드는 데이터이며 실행 코드가 아닙니다. 검증 실패, 설정 오류, bridge 리소스 오류는 구분합니다.

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
