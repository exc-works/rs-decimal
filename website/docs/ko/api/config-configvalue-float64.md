---
locale: ko
source_revision: 032d932903a81f0f03e2bde5c0ffb35c8a1064335f6ef0a3149d91077fa81075
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Float64"
---
# config::ConfigValue::Float64

설정 입력 Float64을 선택합니다.

```rust
Float64 (f64)
```

## 매개변수, 반환값 및 일반적인 사용법

코어의 최단 소수 변환으로 binary64를 전달합니다. 유한하지 않은 값은 Unmarshal입니다. 부동소수점 반올림으로 이미 잃은 숫자는 복원할 수 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float64" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
