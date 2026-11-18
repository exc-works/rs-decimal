---
locale: ko
source_revision: 3576d2b52e6de01c1594ed23748d6a482f290fdb6f5b45fc15ec07a4cf297d34
translation_provenance: en-US-korean-reviewed
api_id: "config::ConfigValue::Bytes"
---
# config::ConfigValue::Bytes

설정 입력 Bytes을 선택합니다.

```rust
Bytes (& 'a [u8])
```

## 매개변수, 반환값 및 일반적인 사용법

바이너리 소수 페이로드가 아니라 소수 텍스트 바이트를 빌립니다. 디코딩 전 브라우저 Uint8Array의 변경을 반영하며 배열을 수정하지 않습니다. 잘못된 UTF-8은 소수 파싱에 실패합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-bytes" locale="ko" />

## 브라우저 제한

config와 serde 기능이 켜져 있습니다. 원문/바이트는 8192 UTF-8 바이트로 제한합니다. Rust는 소수 텍스트와 디코딩한 JSON 문자열을 파싱 전에 검사하여 이스케이프로 [리소스 정책](../resource-policy)을 우회하지 못하게 합니다. 128비트 정수는 범위 안의 bigint가 필요합니다. 빌린 Decimal/NullDecimal 핸들은 디코딩할 때 살아 있어야 합니다. 변환은 입력을 소비하지 않고 독립된 값을 만듭니다. 일반 Serde 트레이트 동작은 구체적인 JSON 어댑터로 보여 줍니다.

## 관련 메서드

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
