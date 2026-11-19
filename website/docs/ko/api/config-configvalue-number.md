---
locale: ko
source_revision: d9b1450ea64f3f424b8c3303a77d21fe7ac3c0fd6d9a478841f3ae7df9627834
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Number"
---
# config::ConfigValue::Number

설정 입력 Number을 선택합니다.

```rust
Number (& 'a str)
```

## 매개변수, 반환값 및 일반적인 사용법

부동소수점 변환 없이 숫자 원문을 전달합니다. 코어는 소수 구문을 파싱하며 이 변형은 전체 JSON 문법을 검증하지 않습니다. 빈 Number는 nullable 대상에서도 잘못된 값입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-number" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
