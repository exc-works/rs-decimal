---
locale: ko
source_revision: 324990894f6327d9f0e88937d867ea0f32bc3c792ba37210279bca0b73495f73
translation_provenance: en-US-korean-reviewed
api_id: "config::decode_nullable"
---
# config::decode_nullable

선택적인 설정 값을 디코딩합니다.

```rust
fn decode_nullable (value : ConfigValue < '_ >) -> Result < NullDecimal , DecimalError >
```

## 매개변수, 반환값 및 일반적인 사용법

value: ConfigValue. `Result<NullDecimal, DecimalError>`/새 nullable 핸들을 반환합니다. Null, 정확히 빈 Text, 빈 Bytes는 내부 미초기화 0을 가진 유효하지 않은 값이 됩니다. 공백만 있는 Text와 빈 Number는 여전히 소수 파싱에 실패합니다. 기존 nullable은 유효하지 않더라도 유효성과 숨겨진 내부 데이터를 복제합니다. 빌린 Decimal은 초기화를 강제하지 않고 유효한 값이 됩니다. 다른 입력은 decode_decimal을 따르고 성공하면 유효해집니다. 입력을 소비하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-nullable" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
