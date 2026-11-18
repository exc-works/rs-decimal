---
locale: ko
source_revision: 1a045fe49700f86179604a439c8da809b0c8210bc17954bf4d71660df81e03c8
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::impl Deserialize < 'de >"
---
# NullDecimal::impl Deserialize < 'de >

일반 Serde visitor로 역직렬화합니다.

```rust
impl< 'de > Deserialize < 'de > for NullDecimal
```

## 매개변수, 반환값 및 일반적인 사용법

deserializer: Deserializer를 구현하는 D. Self 또는 D::Error를 반환합니다. 정확한 소수 문자열, 정수 visitor 입력, null을 받습니다. 부동소수점 visitor 입력은 거부합니다. 소수부/지수가 있는 JSON 숫자는 전용 raw JSON/config 어댑터를 사용해야 합니다. JSON null은 내부 미초기화 0을 가진 유효하지 않은 nullable 값이 됩니다. 빈 문자열은 잘못된 값입니다. 정적 브라우저 어댑터 deserialize(source)는 전체 원본 JSON 텍스트에 serde_json을 실행합니다. 임의의 Rust 역직렬화기 지원보다 범위가 좁습니다. 오류는 category="serde"와 원래 메시지를 제공합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-deserialize-de" locale="ko" />

## 브라우저 제한

config와 serde 기능이 켜져 있습니다. 원문/바이트는 8192 UTF-8 바이트로 제한합니다. Rust는 소수 텍스트와 디코딩한 JSON 문자열을 파싱 전에 검사하여 이스케이프로 [리소스 정책](../resource-policy)을 우회하지 못하게 합니다. 128비트 정수는 범위 안의 bigint가 필요합니다. 빌린 Decimal/NullDecimal 핸들은 디코딩할 때 살아 있어야 합니다. 변환은 입력을 소비하지 않고 독립된 값을 만듭니다. 일반 Serde 트레이트 동작은 구체적인 JSON 어댑터로 보여 줍니다.

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
