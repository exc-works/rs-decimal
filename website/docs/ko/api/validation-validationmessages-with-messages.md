---
locale: ko
source_revision: 4973633c1cdc3ddfb696d250ca871028b9835fa4187520f393245e4690f6a0c3
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationMessages::with_messages"
---
# validation::ValidationMessages::with_messages

영어 검증 템플릿을 재정의합니다.

```rust
fn with_messages (messages : BTreeMap < String , String >) -> Result < Self , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

messages: `BTreeMap<String,String>`/문자열 값 객체. `Result<ValidationMessages, DecimalError>`/메시지 핸들을 반환합니다. 제공한 키만 재정의하며 다른 규칙은 영어 기본값을 유지합니다. 허용 키는 12개 decimal_* 규칙입니다. 빈 템플릿, 알 수 없는 키, 짝이 맞지 않는 중괄호, {0}/{1} 이외의 자리표시자는 InvalidArgument입니다. 유효한 자리표시자의 반복은 허용합니다. 전역 또는 이미 생성한 템플릿을 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-with-messages" locale="ko" />

## 브라우저 제한

validation 기능은 활성화되어 있습니다. 각 문자열은 UTF-8 8192바이트, 규칙/오류 목록과 메시지 맵은 64항목, JSON 전송은 65536바이트까지입니다. 규칙의 소수 매개변수는 [리소스 정책](../resource-policy)을 따릅니다. 번역 전에 치환 텍스트의 보수적인 총합 상한 65536바이트를 검사하므로 실제로는 딱 맞는 결과도 거부할 수 있습니다. 사용자 태그와 필드는 데이터이며 실행 코드가 아닙니다. 검증 실패, 설정 오류, bridge 리소스 오류는 구분합니다.

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
