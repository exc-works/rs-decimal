---
locale: ko
source_revision: 52852655da8fa507f812b593c8e5ba40f3e2342d29c1bda28c254c4a87933fda
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::impl Serialize"
---
# NullDecimal::impl Serialize

Rust Serde 트레이트로 직렬화합니다.

```rust
impl Serialize for NullDecimal
```

## 매개변수, 반환값 및 일반적인 사용법

serializer: Serializer를 구현하는 S. S::Ok/S::Error를 반환합니다. 코어는 정규화된 소수 문자열 또는 null로 직렬화합니다. 유효하지 않은 nullable은 null로 직렬화합니다. 유효한 값의 내부 미초기화 0은 문자열 "0"이 됩니다. 소수부의 후행 0을 제거합니다. 브라우저 어댑터 serialize()는 serde_json을 선택하고 전체 JSON 텍스트를 반환합니다. 임의의 Rust 직렬화기는 제공하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize" locale="ko" />

## 브라우저 제한

config와 serde 기능이 켜져 있습니다. 원문/바이트는 8192 UTF-8 바이트로 제한합니다. Rust는 소수 텍스트와 디코딩한 JSON 문자열을 파싱 전에 검사하여 이스케이프로 [리소스 정책](../resource-policy)을 우회하지 못하게 합니다. 128비트 정수는 범위 안의 bigint가 필요합니다. 빌린 Decimal/NullDecimal 핸들은 디코딩할 때 살아 있어야 합니다. 변환은 입력을 소비하지 않고 독립된 값을 만듭니다. 일반 Serde 트레이트 동작은 구체적인 JSON 어댑터로 보여 줍니다.

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
