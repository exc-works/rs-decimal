---
locale: ko
source_revision: 9e6af4fbdd99e4ba366c644aaf4e403edbed7726d41f29fd990845c2a190132b
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Unsigned"
---
# config::ConfigValue::Unsigned

설정 입력 Unsigned을 선택합니다.

```rust
Unsigned (u128)
```

## 매개변수, 반환값 및 일반적인 사용법

부호 없는 u128을 정확히 전달합니다. 브라우저는 [0, 2^128-1] 범위의 bigint를 요구하며 부동소수점을 거치지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsigned" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
