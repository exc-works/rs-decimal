---
locale: ko
source_revision: 71b55f4d77a06034a780155a4f624e277f9df950e286fd15014d7cc9e03ee581
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

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
