---
locale: ko
source_revision: 57e2d57e143cb2ba5e54935f1231d3095826fead303cbd95c70c73feba2c6bfd
translation_provenance: en-US-korean-reviewed
api_id: hundred
---
# hundred

십진 정수 100을 생성합니다.

```rust
fn hundred () -> Decimal
```

## 매개변수, 반환값, 사용 시 주의점

매개변수 없음. 소수 자릿수 0인 새 초기화 Decimal을 반환합니다. 값을 만드는 생성자이며 공유되는 가변 상수가 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-hundred" locale="ko" />

## 관련 메서드

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
