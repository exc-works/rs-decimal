---
locale: ko
source_revision: 25693aaef850a3e88dcc638a63de0aa32a65fe1057c9bac969c2400862072704
translation_provenance: en-US-korean-reviewed
api_id: Decimal::string
---
# Decimal::string

고정소수점 텍스트로 서식화합니다.

```rust
fn string (& self) -> String
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. 과학 표기 없이 String/string을 반환합니다. 불필요한 소수 부분의 후행 0을 제거하며 정수의 0은 유지합니다. 점을 사용하고 그룹화하지 않습니다. 음의 0을 별도로 표현하지 않습니다. 기본 0은 초기화하지 않고 0으로 출력하며 서식화는 입력을 빌립니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-string" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
