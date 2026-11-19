---
locale: ko
source_revision: 0268b5969d6d0344c17f1c1c3dee925ce05b68c8ea0fc3782a3dde3b820e544c
translation_provenance: en-US-korean-reviewed
api_id: "config::decode_decimal"
---
# config::decode_decimal

명시적으로 형식을 지정한 설정 값을 디코딩합니다.

```rust
fn decode_decimal (value : ConfigValue < '_ >) -> Result < Decimal , DecimalError >
```

## 매개변수, 반환값 및 일반적인 사용법

value: ConfigValue. `Result<Decimal, DecimalError>`/새 Decimal 핸들을 반환합니다. Rust에는 config가 필요합니다. nullable이 아닌 디코딩은 Null과 유효하지 않은 nullable을 숫자 0으로 만들지 않고 거부합니다. 기존 Decimal과 유효한 nullable은 미초기화 0을 포함한 전체 표현을 복제합니다. Text/Number/Bytes는 SQL 큰따옴표를 제거하지 않고 정확한 소수 텍스트를 파싱합니다. 부동소수점은 이미 이진 반올림을 포함하므로 원래 숫자가 정확해야 하면 텍스트나 bigint를 사용하세요. 브라우저 변형은 한 실행에 속하며 디코딩까지 핸들/버퍼를 빌립니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-decimal" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
