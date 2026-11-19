---
locale: ko
source_revision: db21c0833b97369db1efcc9a73b34501c5dcac820064e9658bbb1cb161d8dec2
translation_provenance: en-US-korean-reviewed
api_id: "validation::default_validation_messages"
---
# validation::default_validation_messages

로케일 템플릿의 새 맵을 얻습니다.

```rust
fn default_validation_messages (locale : & str) -> BTreeMap < String , String >
```

## 매개변수, 반환값, 일반적인 사용법

locale: &str/string. 12개 규칙 템플릿을 모두 담은 `BTreeMap<String,String>`/일반 객체를 반환합니다. 알 수 없는 로케일은 영어를 사용합니다. 각 호출은 맵을 독립적으로 소유하므로 수정해도 이후 호출이나 다른 ValidationMessages 객체에 영향이 없습니다. 재내보내기는 같은 함수의 공개 모듈 가져오기 경로입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-default-validation-messages" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
