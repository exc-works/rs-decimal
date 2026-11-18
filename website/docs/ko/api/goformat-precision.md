---
locale: ko
source_revision: b5c0056c42f3f4e6d3613af84d12321baa6359b54407d77f1aacdb7d5d883dd7
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

## 브라우저 제한

[리소스 정책](../resource-policy)은 서식 폭을 65536바이트, 정밀도를 1000으로 제한하며 고정소수점 서식은 계수 증가도 검사합니다. 버퍼는 65536바이트까지입니다. 추가 연산은 보수적인 크기 추정을 사용하므로 실제로는 딱 맞는 연산도 거부할 수 있습니다. 리소스 오류는 입력이나 버퍼를 바꾸지 않습니다. print는 긴 표시 텍스트를 별도로 자릅니다. 전체 결과를 확인하려면 길이를 검사하거나 작은 부분으로 나눠 출력하세요.

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
