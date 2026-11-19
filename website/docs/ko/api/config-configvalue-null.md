---
locale: ko
source_revision: 846797abc20f334aac55a6cc16f18858d3e8f7870cd49af2d7f236e4e0d093b3
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Null"
---
# config::ConfigValue::Null

설정 입력 Null을 선택합니다.

```rust
Null
```

## 매개변수, 반환값 및 일반적인 사용법

누락된 설정 입력입니다. decode_nullable은 유효하지 않은 값을 반환하고 decode_decimal은 Unmarshal로 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-null" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
