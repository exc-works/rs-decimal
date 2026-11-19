---
locale: ko
source_revision: d3605c64b7e680ec58960c25358a321ed68015b81953ff878939d179147793d3
translation_provenance: en-US-korean-reviewed
api_id: "DecimalError::impl Error"
---
# DecimalError::impl Error

Rust Error 구현을 사용합니다.

```rust
impl Error for DecimalError
```

## 매개변수, 반환값, 일반적인 사용법

박스에 담은 오류 전파를 포함해 표준 Rust Error로 사용합니다. 추가 종류는 연결된 원인이 아니며 source()는 None입니다. 네이티브 Rust 트레이트 문법이며 브라우저는 메서드로 공개하지 않습니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimalerror-impl-error" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
