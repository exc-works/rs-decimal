---
locale: ko
source_revision: c7a338ce433accfeb61051f993009577dfae7589a629a17bae6043234f27ca38
translation_provenance: en-US-korean-reviewed
api_id: Decimal::max
---
# Decimal::max

두 수치 중 큰 값을 복사합니다.

```rust
fn max (& self , other : & Self) -> Self
```

## 매개변수, 반환값, 사용 시 주의점

other: &Decimal을 빌려 독립적인 Decimal을 반환합니다. 수치가 같으면 첫 피연산자의 소수 자릿수와 초기화 상태를 유지합니다. Rust에서는 Decimal::max(&left, &right)로 이 빌림 고유 메서드를 명시적으로 선택하세요. Ord::max는 소유권을 가져갑니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-max" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
