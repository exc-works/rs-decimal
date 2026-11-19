---
locale: ko
source_revision: 19e13f576512aa6ef6c12c54900338d87cb485da07e25d15d575e4c5353b491a
translation_provenance: en-US-korean-reviewed
api_id: Decimal::is_not_zero
---
# Decimal::is_not_zero

수치가 0과 다른지 확인합니다.

```rust
fn is_not_zero (& self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

is_zero()의 반대인 bool/boolean을 반환합니다. 미초기화 기본값에는 false를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-not-zero" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
