---
locale: ko
source_revision: 5f1761508e55494581ac3ef80177b3b3480f53a919c2efbe231b66c02ccf3a14
translation_provenance: en-US-korean-reviewed
api_id: GoFormat.precision
---
# GoFormat.precision

precision 서식 필드를 설정합니다.

```rust
precision: Option < usize >
```

## 매개변수, 반환값, 일반적인 사용법

precision의 의미는 서식 문자에 따라 다릅니다. f/F는 소수 자릿수, e/E는 첫 숫자 뒤 자릿수, g/G는 유효 숫자 개수(0은 1로 처리), d는 최소 정수 자릿수입니다. None/null은 각 서식의 기본값을 유지하며 f는 저장된 자릿수, e는 6자리를 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-precision" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
