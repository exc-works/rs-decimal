---
locale: ko
source_revision: 3594148ee83ab0bf68ef800a2a73612702e5a4d6ff495df20f526a7cdddcfcf1
translation_provenance: en-US-korean-reviewed
api_id: "config::deserialize_json_decimal"
---
# config::deserialize_json_decimal

정확한 JSON 설정 필드를 역직렬화합니다.

```rust
fn deserialize_json_decimal < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < Decimal , D :: Error >
```

## 매개변수, 반환값 및 일반적인 사용법

deserializer: Serde Deserializer를 구현하는 D. #[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")]에 사용합니다. serde_json 소스와 config 기능이 필요합니다. 부동소수점을 거치지 않고 원래 숫자 토큰을 유지합니다. 브라우저는 임의의 역직렬화기가 아니라 완전한 JSON 스칼라 원문 문자열용 어댑터를 제공합니다. JSON null과 빈 문자열은 nullable이 아닌 대상에서 거부합니다. 오류는 Serde 역직렬화기의 오류 형식입니다. 브라우저 category="serde"는 DecimalError 종류를 만들어 내지 않고 원래 메시지를 보존합니다. JSON 공백 이외의 후행 입력은 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-decimal" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
