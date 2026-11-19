---
locale: ko
source_revision: 9937fd6977e8a466e16b6a35082706b1183e93c9c873b5eb8e6c66d5ab917b4f
translation_provenance: en-US-korean-reviewed
api_id: "validation::ValidationMessages"
---
# validation::ValidationMessages

소유하는 템플릿으로 검증 실패를 번역합니다.

```rust
pub struct ValidationMessages
```

## 매개변수, 반환값, 일반적인 사용법

ValidationError 레코드를 번역하는 템플릿을 소유합니다. new(locale) 또는 with_messages(map)을 사용합니다. 다른 템플릿 맵을 변경해도 이 핸들에는 영향을 주지 않습니다. 로캘 비교는 대소문자를 구분하지 않으며 하이픈/밑줄과 언어/지역 접미사를 허용합니다. zh-Hant/zh-TW는 번체 중국어를 선택합니다. en-US/en-GB는 영어를 사용하고 알 수 없는 로캘도 영어로 대체합니다. {0}은 필드, {1}은 원래 매개변수를 한 번만 삽입하며 삽입한 중괄호는 다시 확장하지 않습니다. 결과는 일반 텍스트이므로 HTML이 아니라 텍스트로 표시하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
