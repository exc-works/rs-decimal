---
locale: ko
source_revision: b07552403b149d5a66b24c1aa4c0547e8259f4d124ac401ca1161c70929a6aa6
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::marshal_bson_value"
---
# NullDecimal::marshal_bson_value

BSON 타입 바이트와 스칼라 페이로드를 인코딩합니다.

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]를 반환합니다. Rust는 bson 기능이 필요하며 브라우저 빌드에서 활성화되어 있습니다. 튜플은 완전한 BSON 문서가 아니라 원시 BSON 스칼라를 담습니다. 유효한 nullable은 내부가 미초기화 0이어도 BSON 문자열로 인코딩합니다. 무효인 값은 타입 10과 빈 페이로드로 인코딩합니다. 문자열 페이로드는 끝의 NUL을 포함한 리틀엔디언 i32 바이트 길이 뒤에 소수 부분의 후행 0을 제거한 정규 텍스트가 이어집니다. 반환된 페이로드 바이트는 독립적인 복사본이며 인코딩은 초기화나 유효성을 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-nulldecimal-marshal-bson-value" locale="ko" />

## 브라우저 제한

입력은 Uint8Array, kind는 0–255 정수여야 합니다. Wasm 복사 전에 입력을 65536바이트로 제한합니다. Rust는 바이너리 소수 자릿수를 최대 1000, 실제 계수를 최대 16384비트로 검사합니다. BSON 문자열은 8192바이트 텍스트 제한을 따르며 Decimal128 지수 확장도 [리소스 검사](../resource-policy)를 받습니다. 네이티브 라이브러리보다 좁은 제한일 수 있습니다. 검사는 수신 객체 변경 전에 수행하고 코어 오류와 리소스 오류를 구분하며 이동된 핸들은 재사용할 수 없습니다.

## 관련 메서드

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
