---
locale: ko
source_revision: 84f61035965dcc9b605c00e7c877afbafcb7870fb2b3b89e0902348c80e03dd7
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

## 브라우저 제한

[리소스 정책](../resource-policy)은 서식 폭을 65536바이트, 정밀도를 1000으로 제한하며 고정소수점 서식은 계수 증가도 검사합니다. 버퍼는 65536바이트까지입니다. 추가 연산은 보수적인 크기 추정을 사용하므로 실제로는 딱 맞는 연산도 거부할 수 있습니다. 리소스 오류는 입력이나 버퍼를 바꾸지 않습니다. print는 긴 표시 텍스트를 별도로 자릅니다. 전체 결과를 확인하려면 길이를 검사하거나 작은 부분으로 나눠 출력하세요.

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
