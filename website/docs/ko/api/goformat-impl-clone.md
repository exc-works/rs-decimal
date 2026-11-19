---
locale: ko
source_revision: 88974eef4784bf8f1c8b6ec4ce25d3380a2169d449467ecdb4cee7997d0acdd8
translation_provenance: en-US-korean-reviewed
api_id: GoFormat::impl Clone
---
# GoFormat::impl Clone

Rust Clone 구현을 사용합니다.

```rust
impl Clone for GoFormat
```

## 매개변수, 반환값, 일반적인 사용법

GoFormat은 작은 Copy 필드 구조체입니다. Clone/Copy는 옵션을 유지하고 Debug는 필드를 출력합니다. Default는 v, width/precision에 None, 모든 플래그에 false를 선택합니다. 이 Rust 문법은 네이티브에서 검사하며 JavaScript 객체는 일반 JS 필드 객체의 동작과 GoFormat.default()를 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-clone" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
