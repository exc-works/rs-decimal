---
locale: ko
source_revision: 8675e68949343d33893aa6f81cc6a57f32cdafd3aaf80161520178113f46a31d
translation_provenance: en-US-korean-reviewed
api_id: NullDecimal
---
# NullDecimal

유효성과 저장값이 독립적인 선택적 소수를 나타냅니다.

```rust
pub struct NullDecimal
```

## 일반적인 사용법과 의미

값의 존재 여부는 valid만 결정합니다. 무효인 nullable이 0이 아닌 소수를 유지할 수 있으며 유효한 nullable이 미초기화 0을 담을 수도 있습니다. 공개 Rust decimal 필드는 JavaScript에서 복사/이동을 명시한 어댑터로 공개합니다. 기본값은 값 없음입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal" locale="ko" />

공유 예제는 네이티브 Rust와 브라우저에서 실행됩니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
