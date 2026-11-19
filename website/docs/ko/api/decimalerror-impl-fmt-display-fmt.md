---
locale: ko
source_revision: 725ef7f08917b0c306a1f269aaa8e188bc4212046b8491281df04a8f33b0e006
translation_provenance: en-US-korean-reviewed
api_id: "DecimalError::impl fmt :: Display::fmt"
---
# DecimalError::impl fmt :: Display::fmt

Rust fmt 구현을 사용합니다.

```rust
fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result
```

## 매개변수, 반환값, 일반적인 사용법

Display는 메시지만 씁니다. format!이나 to_string()을 사용하고 기계적인 처리는 kinds를 별도로 검사하세요. 네이티브 Rust 트레이트 문법이며 브라우저는 메서드로 공개하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-fmt-display-fmt" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
