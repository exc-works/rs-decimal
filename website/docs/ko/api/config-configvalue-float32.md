---
locale: ko
source_revision: e064dade2acfdfd8dc8224a07d00162d5bdf94c66ca955bda753d76d5540ec7b
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Float32"
---
# config::ConfigValue::Float32

설정 입력 Float32을 선택합니다.

```rust
Float32 (f32)
```

## 매개변수, 반환값 및 일반적인 사용법

binary32를 전달합니다. Rust는 브라우저 number를 먼저 f32로 좁힌 뒤 코어 변환을 사용합니다. 유한하지 않은 입력이나 축소 오버플로는 Unmarshal입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float32" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
