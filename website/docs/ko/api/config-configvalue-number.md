---
locale: ko
source_revision: 74e7f2ffd43f198c10e7ebd972920c487311d6c7ab45db8c6281425ea90f9c14
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Number"
---
# config::ConfigValue::Number

설정 입력 Number을 선택합니다.

```rust
Number (& 'a str)
```

## 매개변수, 반환값 및 일반적인 사용법

부동소수점 변환 없이 숫자 원문을 전달합니다. 코어는 소수 구문을 파싱하며 이 변형은 전체 JSON 문법을 검증하지 않습니다. 빈 Number는 nullable 대상에서도 잘못된 값입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-number" locale="ko" />

## 브라우저 제한

config와 serde 기능이 켜져 있습니다. 원문/바이트는 8192 UTF-8 바이트로 제한합니다. Rust는 소수 텍스트와 디코딩한 JSON 문자열을 파싱 전에 검사하여 이스케이프로 [리소스 정책](../resource-policy)을 우회하지 못하게 합니다. 128비트 정수는 범위 안의 bigint가 필요합니다. 빌린 Decimal/NullDecimal 핸들은 디코딩할 때 살아 있어야 합니다. 변환은 입력을 소비하지 않고 독립된 값을 만듭니다. 일반 Serde 트레이트 동작은 구체적인 JSON 어댑터로 보여 줍니다.

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
