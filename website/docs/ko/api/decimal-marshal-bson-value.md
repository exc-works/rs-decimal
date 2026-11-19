---
locale: ko
source_revision: 44afbeff0d59cbde33e83e7f33c7e22c9903950f7ecf43d6f8230c6cde4b2ec4
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::marshal_bson_value"
---
# Decimal::marshal_bson_value

BSON 타입 바이트와 스칼라 페이로드를 인코딩합니다.

```rust
fn marshal_bson_value (& self) -> Result < (u8 , Vec < u8 >) , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. `Result<(u8, Vec<u8>), DecimalError>`/[number, Uint8Array]를 반환합니다. Rust는 bson 기능이 필요하며 브라우저 빌드에서 활성화되어 있습니다. 튜플은 완전한 BSON 문서가 아니라 원시 BSON 스칼라를 담습니다. 초기화된 값은 Decimal128이 아니라 BSON 문자열(타입 2)로 인코딩합니다. 미초기화 0은 타입 10과 빈 페이로드로 인코딩합니다. 문자열 페이로드는 끝의 NUL을 포함한 리틀엔디언 i32 바이트 길이 뒤에 소수 부분의 후행 0을 제거한 정규 텍스트가 이어집니다. 반환된 페이로드 바이트는 독립적인 복사본이며 인코딩은 초기화나 유효성을 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="binary-bson-decimal-marshal-bson-value" locale="ko" />

## 관련 메서드

[marshal_binary](./decimal-marshal-binary) · [unmarshal_bson_value](./decimal-unmarshal-bson-value) · [unmarshal_json](./decimal-unmarshal-json)
