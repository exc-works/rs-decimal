---
locale: ko
source_revision: 317e1f4c19916312bb2823fc0a526f5311afada3dc44c8d3247bdd5fc0f08111
translation_provenance: en-US-korean-reviewed
api_id: Decimal::impl Ord::cmp
---
# Decimal::impl Ord::cmp

Rust Ord로 소수를 수치 순서대로 비교합니다.

```rust
fn cmp (& self , other : & Self) -> Ordering
```

## 일반적인 사용법과 의미

Ord::cmp는 Ordering을 반환하며 -1/0/1을 반환하는 Decimal::cmp와 다릅니다. 표현 차이는 순서에 영향을 주지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-ord-cmp" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
