---
locale: ko
source_revision: f07542c46478fd9edd09bbf3af9bd03d15608eb33e9ed99beb51b022df098d6a
translation_provenance: en-US-korean-reviewed
api_id: reexport::BigInt
---
# reexport::BigInt

크레이트 루트에서 BigInt를 가져옵니다.

```rust
pub use num_bigint::BigInt as BigInt
```

## 매개변수, 반환값, 일반적인 사용법

공개 Rust 재내보내기이며 새 타입이나 구현이 아닙니다. 가져오기 문법은 네이티브에서 검사합니다. 브라우저에서는 각각 bigint, 정확한 유리수 필드 객체, 전역 most_significant_bit 함수에 대응합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigint" locale="ko" />

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
