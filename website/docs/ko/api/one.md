---
locale: ko
source_revision: 46f70cc4972a4c4ca833f0191402c07a03bbeeb19c10706e521c3bfe03276dbf
translation_provenance: en-US-korean-reviewed
api_id: one
---
# one

십진 정수 1을 생성합니다.

```rust
fn one () -> Decimal
```

## 매개변수, 반환값, 사용 시 주의점

매개변수 없음. 소수 자릿수 0인 새 초기화 Decimal을 반환합니다. 한 결과를 수정하거나 해제해도 이후 호출에는 영향이 없습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-one" locale="ko" />

## 관련 메서드

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
