---
locale: ko
source_revision: f893b87d46c7c5082ca48cb1eae39734e63ca91a57f2f944773172a9baf78fca
translation_provenance: en-US-korean-reviewed
api_id: GoFormat.zero
---
# GoFormat.zero

zero 서식 필드를 설정합니다.

```rust
zero: bool
```

## 매개변수, 반환값, 일반적인 사용법

숫자의 왼쪽 채움 부분을 부호 뒤에서 0으로 채웁니다. left가 true이거나 d에 명시적인 precision이 있으면 무시합니다. 기본값은 false입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-zero" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
