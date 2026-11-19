---
locale: ko
source_revision: dcbc410f5dabf4a62b4cb667b765d034201be05823aeeb413148caad325fb23f
translation_provenance: en-US-korean-reviewed
api_id: Decimal::big_int
---
# Decimal::big_int

스케일 적용 전의 정확한 정수 계수를 복사합니다.

```rust
fn big_int (& self) -> BigInt
```

## 매개변수, 반환값, 사용 시 주의점

Rust는 독립적인 BigInt, bridge는 bigint를 반환합니다. 정수로 버림 변환하는 것과 달리 후행 0이 포함됩니다. 예제는 JSON을 위해 bigint를 텍스트로 출력합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-big-int" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
