---
locale: ko
source_revision: 97b85000fbf7a0fb6537eae68258f927a7739647e41e96de3c0f26f460f2d35a
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::impl fmt :: Display"
---
# Decimal::impl fmt :: Display

Rust 서식 문법으로 소수를 출력합니다.

```rust
impl fmt :: Display for Decimal
```

## 일반적인 사용법과 의미

format!("{}", value), println!, to_string()을 사용하세요. fmt::Formatter는 서식 처리 장치가 제공합니다. 기본 서식은 소수 부분의 후행 0을 제거합니다. 추가 서식 플래그는 서식 그룹에서 설명하며 JavaScript에서는 string()/string_with_trailing_zeros()를 사용합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-fmt-display" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
