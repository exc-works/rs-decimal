---
locale: ko
source_revision: b283786af672117d0c3017e513f97f5df201a379427acab824b383812aa4cf93
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

## 브라우저 제한

config와 serde 기능이 켜져 있습니다. 원문/바이트는 8192 UTF-8 바이트로 제한합니다. Rust는 소수 텍스트와 디코딩한 JSON 문자열을 파싱 전에 검사하여 이스케이프로 [리소스 정책](../resource-policy)을 우회하지 못하게 합니다. 128비트 정수는 범위 안의 bigint가 필요합니다. 빌린 Decimal/NullDecimal 핸들은 디코딩할 때 살아 있어야 합니다. 변환은 입력을 소비하지 않고 독립된 값을 만듭니다. 일반 Serde 트레이트 동작은 구체적인 JSON 어댑터로 보여 줍니다.

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
