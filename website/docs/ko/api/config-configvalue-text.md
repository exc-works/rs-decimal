---
locale: ko
source_revision: 9cc3411fb84577d5ea175fe7b4b5d7e0d13f86f3ba54a79c3a0728b721c31211
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Text"
---
# config::ConfigValue::Text

설정 입력 Text을 선택합니다.

```rust
Text (& 'a str)
```

## 매개변수, 반환값 및 일반적인 사용법

정확한 소수 텍스트를 빌립니다. 바깥 공백은 소수 코어가 처리하며 SQL 형식의 바깥 큰따옴표는 제거하지 않습니다. 정확히 빈 텍스트만 nullable 부재로 취급합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-text" locale="ko" />

## 브라우저 제한

config와 serde 기능이 켜져 있습니다. 원문/바이트는 8192 UTF-8 바이트로 제한합니다. Rust는 소수 텍스트와 디코딩한 JSON 문자열을 파싱 전에 검사하여 이스케이프로 [리소스 정책](../resource-policy)을 우회하지 못하게 합니다. 128비트 정수는 범위 안의 bigint가 필요합니다. 빌린 Decimal/NullDecimal 핸들은 디코딩할 때 살아 있어야 합니다. 변환은 입력을 소비하지 않고 독립된 값을 만듭니다. 일반 Serde 트레이트 동작은 구체적인 JSON 어댑터로 보여 줍니다.

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
