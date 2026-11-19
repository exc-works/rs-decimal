---
locale: ko
source_revision: c5619d120e2ab677bb25ae58cef9205a1160d82cd87b1081f2a6532750a83303
translation_provenance: en-US-korean-reviewed
api_id: GoFormat.space
---
# GoFormat.space

space 서식 필드를 설정합니다.

```rust
space: bool
```

## 매개변수, 반환값, 일반적인 사용법

plus가 false이면 음수가 아닌 숫자 출력 앞에 공백을 붙입니다. 음수는 부호를 유지합니다. 기본값은 false입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-space" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
