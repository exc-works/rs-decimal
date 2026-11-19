---
locale: ko
source_revision: 536eae84e3e9c7c0c702cbcb339f82c64bf0aafab561d4a3f87590939cb52d8e
translation_provenance: en-US-korean-reviewed
api_id: between
---
# between

소수가 양 끝을 포함하는 수치 범위 안에 있는지 확인합니다.

```rust
fn between (v : & Decimal , lower : & Decimal , upper : & Decimal) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

v, lower, upper를 &Decimal로 빌려 lower <= v <= upper의 bool/boolean을 반환합니다. 동등성은 표현 차이를 무시합니다. 경계를 재정렬하지 않으므로 lower > upper이면 false입니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-between" locale="ko" />

## 관련 메서드

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
