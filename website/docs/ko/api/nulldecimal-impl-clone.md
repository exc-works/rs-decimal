---
locale: ko
source_revision: 056afbd5297b8421283bf4270ff73e28ea9129becd2598b31ff7a84ee04dacd5
translation_provenance: en-US-korean-reviewed
api_id: NullDecimal::impl Clone
---
# NullDecimal::impl Clone

Rust에서 유효성과 저장된 소수를 함께 복제합니다.

```rust
impl Clone for NullDecimal
```

## 일반적인 사용법과 의미

valid가 false여도 복제 시 숨겨진 값을 유지합니다. 네이티브 Rust 트레이트이며, 이후 변경을 추적하는 공유 참조가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-clone" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
