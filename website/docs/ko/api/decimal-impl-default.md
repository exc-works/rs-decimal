---
locale: ko
source_revision: 9ab9d90f4fdbf97aeb5b036e4b2c138345a66dab28af2a920054edaddf0608dd
translation_provenance: en-US-korean-reviewed
api_id: Decimal::impl Default
---
# Decimal::impl Default

Rust Default로 미초기화 수치 0을 만듭니다.

```rust
impl Default for Decimal
```

## 일반적인 사용법과 의미

Default는 생성한 0과 수치로는 같지만 is_nil()은 true입니다. bridge는 Decimal.default()를 제공하며, 이 페이지는 Rust 트레이트 자체를 설명합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-default" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
