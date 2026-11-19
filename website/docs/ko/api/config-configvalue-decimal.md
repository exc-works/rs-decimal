---
locale: ko
source_revision: 8d101e011ba2e4f441ebfc6371c73cb22589c6224599cc87e367af949b20dc8f
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Decimal"
---
# config::ConfigValue::Decimal

설정 입력 Decimal을 선택합니다.

```rust
Decimal (& 'a Decimal)
```

## 매개변수, 반환값 및 일반적인 사용법

기존 Decimal을 빌리고 디코딩할 때 계수, 소수 자릿수, 초기화 상태를 복제합니다. 원본은 계속 사용할 수 있습니다. 디코딩 전에 브라우저 원본을 해제하거나 이동하면 빌린 변형이 무효가 됩니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-decimal" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
