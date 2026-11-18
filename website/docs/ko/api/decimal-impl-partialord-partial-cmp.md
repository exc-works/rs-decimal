---
locale: ko
source_revision: 1a22225e09dfc3b3b1f7504680c33552c2bd9a622eca096fdf946a5ba9c5cfaa
translation_provenance: en-US-korean-reviewed
api_id: Decimal::impl PartialOrd::partial_cmp
---
# Decimal::impl PartialOrd::partial_cmp

Rust의 부분 순서로 소수를 비교합니다.

```rust
fn partial_cmp (& self , other : & Self) -> Option < Ordering >
```

## 일반적인 사용법과 의미

모든 Decimal 값은 비교할 수 있으므로 partial_cmp는 Some(Ordering)을 반환합니다. 기본 0과 생성한 0도 비교할 수 있으며, 소수 자릿수가 달라도 None이 되지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord-partial-cmp" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
