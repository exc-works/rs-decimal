---
locale: ko
source_revision: db26ba148bd1b6fe560088f6be1bce313faff866656299f7c24eeb47568b46bf
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::impl Clone"
---
# config::ConfigValue::impl Clone

네이티브 Clone 구현을 사용합니다.

```rust
impl Clone for config::ConfigValue
```

## 매개변수, 반환값 및 일반적인 사용법

ConfigValue는 작은 형식 지정 열거형입니다. Clone/Copy는 참조 대상을 깊게 복사하지 않고 빌린 참조를 유지합니다. Debug는 진단 텍스트이며 안정된 전송 형식이 아닙니다. 네이티브 Rust 트레이트 구문은 별도로 테스트하며 브라우저 메서드가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-clone" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
