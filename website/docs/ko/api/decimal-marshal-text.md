---
locale: ko
source_revision: dabe3e417d155ec62acc296ebb36a3f87bdc044baf1a1ffd4eb27cbaf056b77d
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::marshal_text"
---
# Decimal::marshal_text

Decimal을 text 바이트로 인코딩합니다.

```rust
fn marshal_text (& self) -> Vec < u8 >
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. 정규 ASCII 소수 텍스트를 담은 `Vec<u8>`/Uint8Array를 반환합니다. 소수 부분의 후행 0을 제거하며 기본 Decimal 0은 ASCII 0을 출력합니다. 반환된 바이트는 독립적인 복사본이며 읽기는 상태를 바꾸지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="text-json-decimal-marshal-text" locale="ko" />

## 관련 메서드

[unmarshal_json](./decimal-unmarshal-json) · [marshal_text](./decimal-marshal-text) · [NullDecimal](./nulldecimal)
