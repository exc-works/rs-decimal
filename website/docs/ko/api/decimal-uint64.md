---
locale: ko
source_revision: c526a43d69654fb21b49f56d858016f45f88e7f7f6ec8eaa0c13c3e5e939cd03
translation_provenance: en-US-korean-reviewed
api_id: Decimal::uint64
---
# Decimal::uint64

성공 플래그와 함께 고정 너비 정수로 변환합니다.

```rust
fn uint64 (& self) -> (u64 , bool)
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. (u64, bool)을 반환하며 [bigint, boolean]에 대응합니다. 범위 안의 정수 값은 정수와 true를 반환합니다. 소수나 오버플로는 버린 값 또는 예외가 아니라 (0, false)를 반환합니다. 음수도 (0, false)를 반환합니다. 소수 자릿수만으로 정수 여부가 정해지지 않으므로 12.00은 성공합니다. 실패와 유효한 0을 구분하도록 항상 플래그를 확인하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-uint64" locale="ko" />

## 관련 메서드

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
