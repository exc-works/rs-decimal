---
locale: ko
source_revision: 7fbcc33cd9efa5ed33f322356ce37cb91f352f88d4c3bb3845c8245fa3ded94d
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

## 브라우저 제한

[리소스 정책](../resource-policy)은 서식 폭을 65536바이트, 정밀도를 1000으로 제한하며 고정소수점 서식은 계수 증가도 검사합니다. 버퍼는 65536바이트까지입니다. 추가 연산은 보수적인 크기 추정을 사용하므로 실제로는 딱 맞는 연산도 거부할 수 있습니다. 리소스 오류는 입력이나 버퍼를 바꾸지 않습니다. print는 긴 표시 텍스트를 별도로 자릅니다. 전체 결과를 확인하려면 길이를 검사하거나 작은 부분으로 나눠 출력하세요.

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
