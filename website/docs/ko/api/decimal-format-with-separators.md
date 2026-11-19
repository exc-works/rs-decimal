---
locale: ko
source_revision: c3fbe0927c540d91118929ccf7291b490ab739216719c7655f9f81744f8edff3
translation_provenance: en-US-korean-reviewed
api_id: Decimal::format_with_separators
---
# Decimal::format_with_separators

정수 숫자를 묶고 소수점 구분자를 선택합니다.

```rust
fn format_with_separators (& self , thousands : char , decimal : char) -> String
```

## 매개변수, 반환값, 일반적인 사용법

thousands: char, decimal: char. 정수 부분을 세 자리씩 묶고 저장한 소수 자릿수를 모두 포함하는 String/string을 반환합니다. 각 JS 인수는 보충 문자를 포함하여 정확히 유니코드 스칼라 하나여야 합니다. 단독 서로게이트나 여러 문자 문자열은 거부합니다. thousands의 NUL은 그룹화를 끄고 decimal의 NUL은 점을 선택합니다. 구분자는 리터럴 문자이며 로케일 이름이 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-with-separators" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
