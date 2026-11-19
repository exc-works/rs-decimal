---
locale: ko
source_revision: 9626ec029554d531f95257728b45431467328baf3b8968c06412f5cc58c689ec
translation_provenance: en-US-korean-reviewed
api_id: Decimal::coefficient
---
# Decimal::coefficient

스케일 적용 전의 정확한 정수 계수를 확인합니다.

```rust
fn coefficient (& self) -> & BigInt
```

## 매개변수, 반환값, 사용 시 주의점

Rust는 &BigInt를 빌리며 bridge는 불변 bigint 스냅샷을 반환합니다. 변경을 추적하는 참조가 아닙니다. 계수에는 후행 0이 포함되며 소수의 정수 부분과 다릅니다. 예제는 JSON에서 모든 숫자를 유지하도록 bigint를 텍스트로 출력합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
