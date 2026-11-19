---
locale: ko
source_revision: 07611a32fa9c50a924b808d879544951b6661718e9f82991f25c28bfa7a925e8
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Signed"
---
# config::ConfigValue::Signed

설정 입력 Signed을 선택합니다.

```rust
Signed (i128)
```

## 매개변수, 반환값 및 일반적인 사용법

부호 있는 i128을 정확히 전달합니다. 브라우저는 [-2^127, 2^127-1] 범위의 bigint를 요구합니다. JS number와 범위 밖 값은 변환 전에 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-signed" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
