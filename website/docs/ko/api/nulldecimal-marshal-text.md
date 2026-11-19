---
locale: ko
source_revision: 2b341a7995c77dbb72c2087390bcd56b4208d2c9ec45de8ea22c2ce3f209501a
translation_provenance: en-US-korean-reviewed
api_id: "NullDecimal::marshal_text"
---
# NullDecimal::marshal_text

NullDecimal을 text 바이트로 인코딩합니다.

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. 정규 ASCII 소수 텍스트를 담은 `Vec<u8>`/Uint8Array를 반환합니다. 유효한 값은 정규 소수 텍스트로, 무효인 값은 단어 null이 아니라 빈 바이트로 인코딩합니다. 반환된 바이트는 독립적인 복사본이며 읽기는 상태를 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-nulldecimal-marshal-text" locale="ko" />

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
