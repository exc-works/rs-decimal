---
locale: ko
source_revision: 0989f60fe84cf7be4e7708a7140f9cc16468c21ad57d855b67f896084ddd8606
translation_provenance: en-US-korean-reviewed
api_id: GoFormat.plus
---
# GoFormat.plus

plus 서식 필드를 설정합니다.

```rust
plus: bool
```

## 매개변수, 반환값, 일반적인 사용법

음수가 아닌 숫자 출력에 +를 표시합니다. space보다 우선합니다. 숫자 서식 문자에 적용하며 v/s/q에는 적용하지 않습니다. 음수는 -를 유지합니다. 기본값은 false입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-plus" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
