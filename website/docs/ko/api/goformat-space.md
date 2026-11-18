---
locale: ko
source_revision: 6ee970e3e2507cdd225eb4c314175b5a95b9aa45ac2e64ed4be5fded0c11b425
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

## 브라우저 제한

[리소스 정책](../resource-policy)은 서식 폭을 65536바이트, 정밀도를 1000으로 제한하며 고정소수점 서식은 계수 증가도 검사합니다. 버퍼는 65536바이트까지입니다. 추가 연산은 보수적인 크기 추정을 사용하므로 실제로는 딱 맞는 연산도 거부할 수 있습니다. 리소스 오류는 입력이나 버퍼를 바꾸지 않습니다. print는 긴 표시 텍스트를 별도로 자릅니다. 전체 결과를 확인하려면 길이를 검사하거나 작은 부분으로 나눠 출력하세요.

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
