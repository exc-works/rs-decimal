---
locale: ko
source_revision: 6440a61a644ecdbf15d2da0254f2f46ec6a0bac106e6a427225c0f45a05d250f
translation_provenance: en-US-korean-reviewed
api_id: Decimal::must_non_negative
---
# Decimal::must_non_negative

음수가 아님을 요구하고 초기화된 복사본을 반환합니다.

```rust
fn must_non_negative (& self) -> Self
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. 0을 포함한 음수가 아닌 입력에는 Decimal을 반환합니다. 음수 입력은 Rust에서 문자열 페이로드를 가진 패닉을 일으킵니다. Wasm에서는 복구 가능한 DecimalError가 아니라 트랩이 되어 playground 실행을 끝냅니다. 새 실행을 시작하여 Wasm 상태를 새로 만드세요. 이는 수치에 대한 단언이며 절댓값 변환이 아닙니다. 부호를 바꾸려면 abs를 사용하세요.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-must-non-negative" locale="ko" />

## 관련 메서드

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
