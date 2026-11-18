---
locale: ko
source_revision: 77287e6c4550f824e46a4556e0aaf001ccf92f4e78984f337df5b480f5f48874
translation_provenance: en-US-korean-reviewed
api_id: "config::decode_nullable"
---
# config::decode_nullable

선택적인 설정 값을 디코딩합니다.

```rust
fn decode_nullable (value : ConfigValue < '_ >) -> Result < NullDecimal , DecimalError >
```

## 매개변수, 반환값 및 일반적인 사용법

value: ConfigValue. `Result<NullDecimal, DecimalError>`/새 nullable 핸들을 반환합니다. Null, 정확히 빈 Text, 빈 Bytes는 내부 미초기화 0을 가진 유효하지 않은 값이 됩니다. 공백만 있는 Text와 빈 Number는 여전히 소수 파싱에 실패합니다. 기존 nullable은 유효하지 않더라도 유효성과 숨겨진 내부 데이터를 복제합니다. 빌린 Decimal은 초기화를 강제하지 않고 유효한 값이 됩니다. 다른 입력은 decode_decimal을 따르고 성공하면 유효해집니다. 입력을 소비하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-nullable" locale="ko" />

## 브라우저 제한

config와 serde 기능이 켜져 있습니다. 원문/바이트는 8192 UTF-8 바이트로 제한합니다. Rust는 소수 텍스트와 디코딩한 JSON 문자열을 파싱 전에 검사하여 이스케이프로 [리소스 정책](../resource-policy)을 우회하지 못하게 합니다. 128비트 정수는 범위 안의 bigint가 필요합니다. 빌린 Decimal/NullDecimal 핸들은 디코딩할 때 살아 있어야 합니다. 변환은 입력을 소비하지 않고 독립된 값을 만듭니다. 일반 Serde 트레이트 동작은 구체적인 JSON 어댑터로 보여 줍니다.

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
