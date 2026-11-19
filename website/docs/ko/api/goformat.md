---
locale: ko
source_revision: f89bca6a9b41c5dfaa6d534efc134c1f42c007ba065d09915785753b5a161115
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

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
