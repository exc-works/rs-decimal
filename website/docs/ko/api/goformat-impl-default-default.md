---
locale: ko
source_revision: 3ce1c461a5149267a6d290d24a336549ce4f3b1fe8eb45cc970b59fa53adfd15
translation_provenance: en-US-korean-reviewed
api_id: GoFormat::impl Default::default
---
# GoFormat::impl Default::default

Rust Default::default 구현을 사용합니다.

```rust
fn default () -> Self
```

## 매개변수, 반환값, 일반적인 사용법

GoFormat은 작은 Copy 필드 구조체입니다. Clone/Copy는 옵션을 유지하고 Debug는 필드를 출력합니다. Default는 v, width/precision에 None, 모든 플래그에 false를 선택합니다. 이 Rust 문법은 네이티브에서 검사하며 bridge 객체는 일반 JS 필드 객체의 동작과 GoFormat.default()를 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="formatting-goformat-impl-default-default" locale="ko" />

## 브라우저 제한

[리소스 정책](../resource-policy)은 서식 폭을 65536바이트, 정밀도를 1000으로 제한하며 고정소수점 서식은 계수 증가도 검사합니다. 버퍼는 65536바이트까지입니다. 추가 연산은 보수적인 크기 추정을 사용하므로 실제로는 딱 맞는 연산도 거부할 수 있습니다. 리소스 오류는 입력이나 버퍼를 바꾸지 않습니다. print는 긴 표시 텍스트를 별도로 자릅니다. 전체 결과를 확인하려면 길이를 검사하거나 작은 부분으로 나눠 출력하세요.

## 관련 메서드

[GoFormat](./goformat) · [format_go](./decimal-format-go) · [append_text](./decimal-append-text) · [string_with_trailing_zeros](./decimal-string-with-trailing-zeros)
