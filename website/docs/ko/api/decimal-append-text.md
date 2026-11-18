---
locale: ko
source_revision: bf26b0f6b02acd5afbbeb54a43a96410bc32ae1ee6825c86870266f2a294720f
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

## 브라우저 제한

[리소스 정책](../resource-policy)은 서식 폭을 65536바이트, 정밀도를 1000으로 제한하며 고정소수점 서식은 계수 증가도 검사합니다. 버퍼는 65536바이트까지입니다. 추가 연산은 보수적인 크기 추정을 사용하므로 실제로는 딱 맞는 연산도 거부할 수 있습니다. 리소스 오류는 입력이나 버퍼를 바꾸지 않습니다. print는 긴 표시 텍스트를 별도로 자릅니다. 전체 결과를 확인하려면 길이를 검사하거나 작은 부분으로 나눠 출력하세요.

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
