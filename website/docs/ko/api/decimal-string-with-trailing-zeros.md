---
locale: ko
source_revision: f320c1a30df05777794b5d83edd6494fed5ebfe3f6b80e1d48ebc83fa5e17748
translation_provenance: en-US-korean-reviewed
api_id: Decimal::string_with_trailing_zeros
---
# Decimal::string_with_trailing_zeros

고정소수점 텍스트로 서식화합니다.

```rust
fn string_with_trailing_zeros (& self) -> String
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. 과학 표기 없이 String/string을 반환합니다. 후행 0을 포함해 저장된 모든 소수 자릿수를 유지합니다. 점을 사용하고 그룹화하지 않습니다. 음의 0을 별도로 표현하지 않습니다. 기본 0은 초기화하지 않고 0으로 출력하며 서식화는 입력을 빌립니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string-with-trailing-zeros" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
