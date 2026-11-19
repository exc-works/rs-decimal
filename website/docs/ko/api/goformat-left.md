---
locale: ko
source_revision: 9b1c2ac4982ef348ecf9982f858e7f552532e21dc0129fa816673741276acaf7
translation_provenance: en-US-korean-reviewed
api_id: GoFormat.left
---
# GoFormat.left

left 서식 필드를 설정합니다.

```rust
left: bool
```

## 매개변수, 반환값, 일반적인 사용법

오른쪽을 공백으로 채웁니다. 왼쪽 정렬은 숫자 0 채우기보다 우선합니다. 기본값은 false입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-left" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
