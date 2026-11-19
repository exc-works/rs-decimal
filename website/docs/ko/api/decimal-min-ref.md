---
locale: ko
source_revision: c991ff009b87a451c4d3a01a7e163c4b8e995107fc18ca0a0ef4ec120e3aee71
translation_provenance: en-US-korean-reviewed
api_id: Decimal::min_ref
---
# Decimal::min_ref

수치 비교로 원래 피연산자를 선택합니다.

```rust
fn min_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## 매개변수, 반환값, 일반적인 사용법

other: &Decimal. Rust는 입력 수명에 맞는 공유 참조를 반환합니다. 값이 같으면 self를 선택하여 소수 자릿수와 초기화 상태를 유지합니다. JavaScript는 복사본이 아니라 해당 원래 래퍼를 반환합니다. 래퍼를 이동하거나 해제하면 모든 JS 별칭도 무효가 됩니다. JS는 Rust 빌림 수명을 강제할 수 없습니다. 독립적으로 소유하는 결과에는 max/min을 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-min-ref" locale="ko" />

## 관련 메서드

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
