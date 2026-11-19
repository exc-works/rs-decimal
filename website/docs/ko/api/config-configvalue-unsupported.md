---
locale: ko
source_revision: dafd4579c57c7280a7d3e288044769361224c3db24eb3a6174eb3833dfe2bd19
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Unsupported"
---
# config::ConfigValue::Unsupported

설정 입력 Unsupported을 선택합니다.

```rust
Unsupported
```

## 매개변수, 반환값 및 일반적인 사용법

미지원 설정 형식을 강제 변환 없이 나타냅니다. 두 디코더 모두 Unmarshal을 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsupported" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
