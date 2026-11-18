---
locale: ko
source_revision: e777d7a0d2eada54e45b9094274df294b6fb9f6fd490746dfc2797aa8d3acb40
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

## 브라우저 제한

config와 serde 기능이 켜져 있습니다. 원문/바이트는 8192 UTF-8 바이트로 제한합니다. Rust는 소수 텍스트와 디코딩한 JSON 문자열을 파싱 전에 검사하여 이스케이프로 [리소스 정책](../resource-policy)을 우회하지 못하게 합니다. 128비트 정수는 범위 안의 bigint가 필요합니다. 빌린 Decimal/NullDecimal 핸들은 디코딩할 때 살아 있어야 합니다. 변환은 입력을 소비하지 않고 독립된 값을 만듭니다. 일반 Serde 트레이트 동작은 구체적인 JSON 어댑터로 보여 줍니다.

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
