---
locale: ko
source_revision: 711eb90b21b1b79f0303fceb3e98f79a6b9b78a5eb632f30a661f662027c1167
translation_provenance: en-US-korean-reviewed
api_id: GoFormat.alternate
---
# GoFormat.alternate

alternate 서식 필드를 설정합니다.

```rust
alternate: bool
```

## 매개변수, 반환값, 일반적인 사용법

f/F/e/E에서 소수점을 유지하고 g/G의 일반 서식에서 후행 0을 유지합니다. 다른 진법을 뜻하지 않습니다. 기본값은 false입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-alternate" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
