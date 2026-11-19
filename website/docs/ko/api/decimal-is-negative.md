---
locale: ko
source_revision: 663e2e4ad6a9b90feffe429f81f13e798f3070cf1d9ea7de1079b0dc756cf5d2
translation_provenance: en-US-korean-reviewed
api_id: Decimal::is_negative
---
# Decimal::is_negative

수치가 엄격히 음수인지 확인합니다.

```rust
fn is_negative (& self) -> bool
```

## 매개변수, 반환값, 사용 시 주의점

bool/boolean을 반환합니다. 미초기화 기본값을 포함한 0에는 false를 반환합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-negative" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
