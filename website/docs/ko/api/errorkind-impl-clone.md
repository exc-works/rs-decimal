---
locale: ko
source_revision: b4b7e965750a7655fdeb57385feedb3c62874c4ab66fdfb6a3f2062d579960d6
translation_provenance: en-US-korean-reviewed
api_id: "ErrorKind::impl Clone"
---
# ErrorKind::impl Clone

Rust Clone 구현을 사용합니다.

```rust
impl Clone for ErrorKind
```

## 매개변수, 반환값, 일반적인 사용법

Clone은 전체 값을 유지하지만 빌린 ScanValue 슬라이스는 여전히 원래 데이터를 참조합니다. 네이티브 Rust 트레이트 문법이며 브라우저는 메서드로 공개하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-errorkind-impl-clone" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
