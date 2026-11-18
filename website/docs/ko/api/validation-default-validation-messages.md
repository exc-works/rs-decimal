---
locale: ko
source_revision: 9f6709052cffc0b28f6c72e94064cf76722563f48008476b3a2d47db292f5bda
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

## 브라우저 제한

validation 기능은 활성화되어 있습니다. 각 문자열은 UTF-8 8192바이트, 규칙/오류 목록과 메시지 맵은 64항목, JSON 전송은 65536바이트까지입니다. 규칙의 소수 매개변수는 [리소스 정책](../resource-policy)을 따릅니다. 번역 전에 치환 텍스트의 보수적인 총합 상한 65536바이트를 검사하므로 실제로는 딱 맞는 결과도 거부할 수 있습니다. 사용자 태그와 필드는 데이터이며 실행 코드가 아닙니다. 검증 실패, 설정 오류, bridge 리소스 오류는 구분합니다.

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
