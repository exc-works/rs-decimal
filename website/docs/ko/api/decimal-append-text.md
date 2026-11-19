---
locale: ko
source_revision: dd4986a8dd71923263f954c74b5685b1dfaa3a4b594ef1aadb36c8a62519bd62
translation_provenance: en-US-korean-reviewed
api_id: Decimal::append_text
---
# Decimal::append_text

기존 가변 버퍼에 소수 ASCII 바이트를 추가합니다.

```rust
fn append_text (& self , dst : & mut Vec < u8 >)
```

## 매개변수, 반환값, 일반적인 사용법

`dst: &mut Vec<u8>`를 ByteBuffer 래퍼로 나타냅니다. Rust는 (), JS는 undefined를 반환합니다. 기존 바이트를 유지하며 대상 버퍼를 제자리에서 바꿉니다. 소수 부분의 후행 0을 제거합니다. Decimal과 버퍼는 계속 사용할 수 있습니다. Uint8Array는 ByteBuffer.new로 복사하며 to_uint8_array는 Wasm 메모리 뷰가 아니라 복사본을 반환합니다. 리소스 실패는 변경 전에 발생하여 버퍼를 그대로 유지합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-append-text" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
