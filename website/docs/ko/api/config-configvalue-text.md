---
locale: ko
source_revision: 89abd084e8bb8541133194e2046ba4621ebe68fb1c342334ebc3bf5d1ca7dcec
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Text"
---
# config::ConfigValue::Text

설정 입력 Text을 선택합니다.

```rust
Text (& 'a str)
```

## 매개변수, 반환값 및 일반적인 사용법

정확한 소수 텍스트를 빌립니다. 바깥 공백은 소수 코어가 처리하며 SQL 형식의 바깥 큰따옴표는 제거하지 않습니다. 정확히 빈 텍스트만 nullable 부재로 취급합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-text" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
