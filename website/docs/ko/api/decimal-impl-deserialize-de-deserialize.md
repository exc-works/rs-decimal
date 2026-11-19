---
locale: ko
source_revision: 24e00c21cccbe1d37441ecf7320c640c608396a025aad09f2a518e2a7e914703
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::impl Deserialize < 'de >::deserialize"
---
# Decimal::impl Deserialize < 'de >::deserialize

일반 Serde visitor로 역직렬화합니다.

```rust
fn deserialize < D : Deserializer < 'de > > (d : D) -> Result < Self , D :: Error >
```

## 매개변수, 반환값 및 일반적인 사용법

deserializer: Deserializer를 구현하는 D. Self 또는 D::Error를 반환합니다. 정확한 소수 문자열, 정수 visitor 입력, null을 받습니다. 부동소수점 visitor 입력은 거부합니다. 소수부/지수가 있는 JSON 숫자는 전용 raw JSON/config 어댑터를 사용해야 합니다. JSON null은 미초기화 Decimal 0이 됩니다. 빈 문자열은 잘못된 값입니다. 정적 브라우저 어댑터 deserialize(source)는 전체 원본 JSON 텍스트에 serde_json을 실행합니다. 임의의 Rust 역직렬화기 지원보다 범위가 좁습니다. 오류는 category="serde"와 원래 메시지를 제공합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-deserialize-de-deserialize" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
