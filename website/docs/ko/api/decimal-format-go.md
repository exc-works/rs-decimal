---
locale: ko
source_revision: f9b1eaa0c6a10e9f32908baa1f2c09a9aafeb3265bffa095bb0a0b210e1e1f05
translation_provenance: en-US-korean-reviewed
api_id: Decimal::format_go
---
# Decimal::format_go

Go 호환 서식 문자 하나와 서식 옵션을 적용합니다.

```rust
fn format_go (& self , f : GoFormat) -> Result < String , DecimalError >
```

## 매개변수, 반환값, 일반적인 사용법

f: GoFormat. Result<String, DecimalError>/string을 반환합니다. %.2f 같은 printf 패턴이 아니라 서식 문자 하나를 받습니다. 지원 문자는 v/s/q/d/f/F/e/E/g/G/b입니다. 최근접 짝수 반올림을 사용합니다. 미지원 문자나 정수가 아닌 d는 예외 대신 %! 표시를 텍스트로 출력합니다. 네이티브 부호 있는 범위를 넘는 고정 정밀도는 InvalidPrecision이며 브라우저의 폭/정밀도/할당 제한은 별도입니다. 생략한 JS 필드는 Rust 기본값을 사용하고 알 수 없는 필드나 잘못된 타입은 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-decimal-format-go" locale="ko" />

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
