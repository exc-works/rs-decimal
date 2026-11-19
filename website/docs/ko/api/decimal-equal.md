---
locale: ko
source_revision: 05d4c976c4f23a3286f06a831a12b99e1cda6e2ba4014722196a6ec9aade2d22
translation_provenance: en-US-korean-reviewed
api_id: Decimal::equal
---
# Decimal::equal

두 소수가 같은 수치를 나타내는지 확인합니다.

```rust
fn equal (& self , other : & Self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

other: &Decimal을 빌려 bool/boolean을 반환합니다. 소수 자릿수와 초기화 상태를 무시합니다. JavaScript의 ===는 래퍼의 동일성을 비교하므로 Rust의 수치 동등성은 equal()로 확인하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-equal" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
