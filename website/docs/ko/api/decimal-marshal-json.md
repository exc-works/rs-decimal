---
locale: ko
source_revision: 76052d80c386b89ac2428898d57b9ce525f22161fd5cd3ee7956cceafe3b11e9
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::marshal_json"
---
# Decimal::marshal_json

Decimal을 json 바이트로 인코딩합니다.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. JSON 바이트를 담은 `Vec<u8>`/Uint8Array를 반환합니다. Rust는 json 기능이 필요하며 브라우저 빌드에서 활성화합니다. 초기화된 Decimal은 따옴표로 감싼 정규 문자열, 기본 미초기화 0은 JSON null로 인코딩합니다. 반환된 바이트는 독립적인 복사본이며 읽기는 상태를 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-json" locale="ko" />

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
