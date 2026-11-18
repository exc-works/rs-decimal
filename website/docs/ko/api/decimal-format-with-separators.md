---
locale: ko
source_revision: c98261b50ba3febe344cea19fd963c1739045fe7ff1872b278b97c36e7fef20b
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

## 브라우저 제한

[리소스 정책](../resource-policy)은 서식 폭을 65536바이트, 정밀도를 1000으로 제한하며 고정소수점 서식은 계수 증가도 검사합니다. 버퍼는 65536바이트까지입니다. 추가 연산은 보수적인 크기 추정을 사용하므로 실제로는 딱 맞는 연산도 거부할 수 있습니다. 리소스 오류는 입력이나 버퍼를 바꾸지 않습니다. print는 긴 표시 텍스트를 별도로 자릅니다. 전체 결과를 확인하려면 길이를 검사하거나 작은 부분으로 나눠 출력하세요.

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
