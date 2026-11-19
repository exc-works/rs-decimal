---
locale: ko
source_revision: 5a5d502f0868797711c2e17ee061bb453dff465719822a493b3a33d226cf87b4
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Nullable"
---
# config::ConfigValue::Nullable

설정 입력 Nullable을 선택합니다.

```rust
Nullable (& 'a NullDecimal)
```

## 매개변수, 반환값 및 일반적인 사용법

기존 nullable 값을 빌립니다. decode_nullable은 유효성과 숨겨진 데이터를 복제합니다. decode_decimal은 유효한 값만 받아 내부 Decimal을 복제합니다. 디코딩 전의 변경을 반영합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-nullable" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
