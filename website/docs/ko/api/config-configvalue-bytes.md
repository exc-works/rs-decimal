---
locale: ko
source_revision: bfcaf340b0c1c1671610ce3a1025921897c59afd9bdf91da4be0b86f5f4d7446
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Bytes"
---
# config::ConfigValue::Bytes

설정 입력 Bytes을 선택합니다.

```rust
Bytes (& 'a [u8])
```

## 매개변수, 반환값 및 일반적인 사용법

바이너리 소수 페이로드가 아니라 소수 텍스트 바이트를 빌립니다. 디코딩 전 브라우저 Uint8Array의 변경을 반영하며 배열을 수정하지 않습니다. 잘못된 UTF-8은 소수 파싱에 실패합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-bytes" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
