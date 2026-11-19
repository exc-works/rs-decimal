---
locale: ko
source_revision: 53a002a881140d83f1a447d3f2f73f066b75a4b8a71f55b044742e2243254251
translation_provenance: en-US-korean-reviewed
api_id: "validation::validate_decimal"
---
# validation::validate_decimal

규칙 실패를 순서대로 모아 번역합니다.

```rust
fn validate_decimal (field : & str , value : & Decimal , rules : & [ValidationRule] ,) -> Vec < ValidationError >
```

## 매개변수, 반환값, 일반적인 사용법

Rust에서는 브라우저 빌드에 켜져 있는 validation 기능이 필요합니다. field: &str/string, value: &Decimal/Decimal, rules: &[ValidationRule]/배열. 실패한 모든 ValidationError를 호출자가 지정한 순서로 반환합니다. 중복 규칙은 중복 오류를 만들 수 있습니다. 리플렉션, 레지스트리, 조기 중단은 없습니다. 빈 규칙은 빈 배열을 반환합니다. 모든 입력을 빌립니다. 필드 이름은 호출자의 텍스트이며 번역 시 그대로 삽입합니다. 누락된 입력을 실패로 처리하려면 required를 명시하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validate-decimal" locale="ko" />

## 관련 메서드

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
