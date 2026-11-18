---
locale: ko
source_revision: 25b1ec5a4bb0557629fac6d66ca48c015418489e6280f42443d66425e8372440
translation_provenance: en-US-korean-reviewed
api_id: Decimal::impl PartialOrd
---
# Decimal::impl PartialOrd

Rust의 부분 순서로 소수를 비교합니다.

```rust
impl PartialOrd for Decimal
```

## 일반적인 사용법과 의미

모든 Decimal 값은 비교할 수 있으므로 partial_cmp는 Some(Ordering)을 반환합니다. 기본 0과 생성한 0도 비교할 수 있으며, 소수 자릿수가 달라도 None이 되지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord" locale="ko" />

이 Rust 문법 예제는 네이티브 테스트에서 실행하며 브라우저의 트레이트 지원을 뜻하지 않습니다.

## 관련 문서

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
