---
locale: ko
source_revision: 0c3a3cc5d3969cdd2cb971de3686daa945e18486f55f8c99179deb863727c501
translation_provenance: en-US-korean-reviewed
api_id: GoFormat.verb
---
# GoFormat.verb

verb 서식 필드를 설정합니다.

```rust
verb: char
```

## 매개변수, 반환값, 일반적인 사용법

유니코드 스칼라 하나를 서식 문자로 선택합니다. 기본 v는 간결한 고정소수점 텍스트, e/E는 과학 표기, q는 따옴표, b는 이진 계수 뒤에 p-scale을 출력합니다(IEEE 부동소수점이 아닙니다).

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-verb" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
