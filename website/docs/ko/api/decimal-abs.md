---
locale: ko
source_revision: c71c6b4ce225ea223944d584a5ec03b1adfe1dbd2adeb04f6beb38800f1e252e
translation_provenance: en-US-korean-reviewed
api_id: Decimal::abs
---
# Decimal::abs

절댓값을 담은 독립적인 소수를 만듭니다.

```rust
fn abs (& self) -> Self
```

## 매개변수, 반환값, 사용 시 주의점

Decimal을 반환하고 소수 자릿수와 후행 0을 유지합니다. 입력이 기본값이어도 결과는 초기화되며 입력은 변하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
