---
locale: ko
source_revision: fe33f9fd8631eeeae5a499a47abbc262340fc190757785abe0749a14c97bf791
translation_provenance: en-US-korean-reviewed
api_id: GoFormat.width
---
# GoFormat.width

width 서식 필드를 설정합니다.

```rust
width: Option < usize >
```

## 매개변수, 반환값, 일반적인 사용법

최소 폭에는 부호와 소수점이 포함됩니다. 더 긴 텍스트를 자르지 않고 부족한 만큼 채웁니다. None/null은 최소 폭을 비활성화합니다. 코어 서식은 출력 바이트 수를 셉니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-width" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
