---
locale: ko
source_revision: f90f9a719348f32512d576fc2ef23076c6492acd11b0dbe87ae7f8c72a7c2eab
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::marshal_json"
---
# NullDecimal::marshal_json

NullDecimal을 json 바이트로 인코딩합니다.

```rust
fn marshal_json (& self) -> Vec < u8 >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. JSON 바이트를 담은 `Vec<u8>`/Uint8Array를 반환합니다. Rust는 json 기능이 필요하며 브라우저 빌드에서 활성화합니다. 무효인 nullable은 JSON null이며 유효한 값은 내부 Decimal이 미초기화여도 따옴표로 감싼 정규 소수입니다. 반환된 바이트는 독립적인 복사본이며 읽기는 상태를 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-json" locale="ko" />

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
