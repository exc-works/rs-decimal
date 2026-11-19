---
locale: ko
source_revision: 03da42d48c5d8b250daaaa93d59162466d4ca8b88da11e32c7b577b2d36d0747
translation_provenance: en-US-korean-reviewed
api_id: "config"
---
# config

명시적으로 형식을 지정한 설정 값을 디코딩합니다.

```rust
pub mod config
```

## 매개변수, 반환값 및 일반적인 사용법

형식 지정 설정 어댑터는 텍스트/텍스트 바이트, 정확한 숫자 텍스트, 부호 있는/없는 128비트 정수, 부동소수점, 빌린 Decimal/NullDecimal 값을 지원합니다. Rust에는 config가 필요합니다. nullable이 아닌 디코딩은 Null과 유효하지 않은 nullable을 숫자 0으로 만들지 않고 거부합니다. 기존 Decimal과 유효한 nullable은 미초기화 0을 포함한 전체 표현을 복제합니다. Text/Number/Bytes는 SQL 큰따옴표를 제거하지 않고 정확한 소수 텍스트를 파싱합니다. 부동소수점은 이미 이진 반올림을 포함하므로 원래 숫자가 정확해야 하면 텍스트나 bigint를 사용하세요. 브라우저 변형은 한 실행에 속하며 디코딩까지 핸들/버퍼를 빌립니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config" locale="ko" />

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
