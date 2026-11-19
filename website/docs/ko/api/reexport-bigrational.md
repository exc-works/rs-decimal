---
locale: ko
source_revision: f67e1caa0fb769c278b4d8058441646a0cf632963d9da4fd8b19596b64289dc8
translation_provenance: en-US-korean-reviewed
api_id: reexport::BigRational
---
# reexport::BigRational

크레이트 루트에서 BigRational를 가져옵니다.

```rust
pub use num_rational::BigRational as BigRational
```

## 매개변수, 반환값, 일반적인 사용법

공개 Rust 재내보내기이며 새 타입이나 구현이 아닙니다. 가져오기 문법은 네이티브에서 검사합니다. 브라우저에서는 각각 bigint, 정확한 유리수 필드 객체, 전역 most_significant_bit 함수에 대응합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-reexport-bigrational" locale="ko" />

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
