---
locale: ko
source_revision: 0ad883afdae2ec6d62af55006e383eb095f5c7928dcdbf6c2f7baf446a49d945
translation_provenance: en-US-korean-reviewed
api_id: GoFormat.width
---
# GoFormat.width

width 서식 필드를 설정합니다.

```rust
width: Option < usize >
```

## 매개변수, 반환값, 일반적인 사용법

최소 폭에는 부호와 소수점이 포함됩니다. 더 긴 텍스트를 자르지 않고 부족한 만큼 채웁니다. None/null은 최소 폭을 비활성화합니다. 코어 서식은 출력 바이트 수를 셉니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-width" locale="ko" />

## 브라우저 제한

[리소스 정책](../resource-policy)은 서식 폭을 65536바이트, 정밀도를 1000으로 제한하며 고정소수점 서식은 계수 증가도 검사합니다. 버퍼는 65536바이트까지입니다. 추가 연산은 보수적인 크기 추정을 사용하므로 실제로는 딱 맞는 연산도 거부할 수 있습니다. 리소스 오류는 입력이나 버퍼를 바꾸지 않습니다. print는 긴 표시 텍스트를 별도로 자릅니다. 전체 결과를 확인하려면 길이를 검사하거나 작은 부분으로 나눠 출력하세요.

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
