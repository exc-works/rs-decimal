---
locale: ko
source_revision: 84828af3d16c67c7fecd4ce94d987a7814ff2080c1c08e3527725c6cb1588c42
translation_provenance: en-US-korean-reviewed
api_id: Decimal::neg
---
# Decimal::neg

부호가 반대인 독립적인 소수를 만듭니다.

```rust
fn neg (& self) -> Self
```

## 매개변수, 반환값, 사용 시 주의점

Decimal을 반환하고 소수 자릿수와 후행 0을 유지합니다. 0은 수치상 0을 유지하며 기본 입력이어도 결과는 초기화됩니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="ko" />

## 관련 메서드

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
