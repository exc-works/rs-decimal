---
locale: ko
source_revision: e9b605c9c2b762d25f61c081bee35aa1c375be0cb3cab1e12e2bd0938c7a3c70
translation_provenance: en-US-korean-reviewed
api_id: GoFormat
---
# GoFormat

필드 객체로 한 번의 서식 연산을 지정합니다.

```rust
pub struct GoFormat
```

## 매개변수, 반환값, 일반적인 사용법

필드는 Rust 구조체의 verb, width, precision, left, plus, space, zero, alternate와 같습니다. bridge의 GoFormat.default()는 Rust 기본값을 읽어 새 가변 객체를 반환합니다. 이를 바꿔도 전역 서식을 설정하지 않습니다. width/precision은 None에 null, Some에 음수가 아닌 wasm32 정수 number를 사용합니다. Rust 트레이트 문법은 네이티브 전용입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat" locale="ko" />

## 브라우저 제한

[리소스 정책](../resource-policy)은 서식 폭을 65536바이트, 정밀도를 1000으로 제한하며 고정소수점 서식은 계수 증가도 검사합니다. 버퍼는 65536바이트까지입니다. 추가 연산은 보수적인 크기 추정을 사용하므로 실제로는 딱 맞는 연산도 거부할 수 있습니다. 리소스 오류는 입력이나 버퍼를 바꾸지 않습니다. print는 긴 표시 텍스트를 별도로 자릅니다. 전체 결과를 확인하려면 길이를 검사하거나 작은 부분으로 나눠 출력하세요.

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
