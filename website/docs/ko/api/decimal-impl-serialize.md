---
locale: ko
source_revision: fd8204204097a2ca3064943a4cf9e6b865ae769705e2693bc4a79f71be2a4582
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::impl Serialize"
---
# Decimal::impl Serialize

Rust Serde 트레이트로 직렬화합니다.

```rust
impl Serialize for Decimal
```

## 매개변수, 반환값 및 일반적인 사용법

serializer: Serializer를 구현하는 S. S::Ok/S::Error를 반환합니다. 코어는 정규화된 소수 문자열 또는 null로 직렬화합니다. 미초기화 Decimal은 null, 초기화된 0은 문자열 "0"으로 직렬화합니다. 소수부의 후행 0을 제거합니다. 브라우저 어댑터 serialize()는 serde_json을 선택하고 전체 JSON 텍스트를 반환합니다. 임의의 Rust 직렬화기는 제공하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-serialize" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
