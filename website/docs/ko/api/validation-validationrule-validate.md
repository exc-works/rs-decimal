---
locale: ko
source_revision: 92b27b2e44b2095f2d604aead4c4f01fb922094a0083fe1c3c5e2509eaa4b3d7
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationRule::validate"
---
# validation::ValidationRule::validate

하나의 필드를 규칙으로 검증합니다.

```rust
fn validate (& self , field : & str , value : & Decimal) -> Result < () , ValidationError >
```

## 매개변수, 반환값, 일반적인 사용법

field: &str/string, value: &Decimal/Decimal. `Result<(), ValidationError>`를 반환합니다. JavaScript에서 성공은 undefined입니다. 실패하면 category="validation"과 field/tag/parameter 속성을 가진 JS ValidationError를 던지며 DecimalError와 다릅니다. 필드와 원래 매개변수 텍스트는 중괄호와 공백까지 유지합니다. 규칙과 값은 빌립니다. 예외 없이 여러 실패를 모으려면 validate_decimal을 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-validate" locale="ko" />

## 브라우저 제한

validation 기능은 활성화되어 있습니다. 각 문자열은 UTF-8 8192바이트, 규칙/오류 목록과 메시지 맵은 64항목, JSON 전송은 65536바이트까지입니다. 규칙의 소수 매개변수는 [리소스 정책](../resource-policy)을 따릅니다. 번역 전에 치환 텍스트의 보수적인 총합 상한 65536바이트를 검사하므로 실제로는 딱 맞는 결과도 거부할 수 있습니다. 사용자 태그와 필드는 데이터이며 실행 코드가 아닙니다. 검증 실패, 설정 오류, bridge 리소스 오류는 구분합니다.

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
