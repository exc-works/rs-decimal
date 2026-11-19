---
locale: ko
source_revision: 69334c1aa4ddf9b63780cdd0f7b3c783c6e1646067bf4562ab026efd47279431
translation_provenance: en-US-korean-reviewed
api_id: "ScanValue::Bytes"
---
# ScanValue::Bytes

SQL 입력 변형 Bytes을 선택합니다.

```rust
Bytes (& 'a [u8])
```

## 매개변수, 반환값, 일반적인 사용법

바이너리/BSON 전송 데이터가 아니라 소수 텍스트 바이트를 빌립니다. 브라우저는 scan까지 Uint8Array를 유지하며 그 전의 수정을 반영합니다. scan은 입력을 바꾸지 않습니다. 잘못된 UTF-8은 소수 파싱에 실패합니다. 변형은 scan에 빌려 전달하며 한 브라우저 실행 안에서 재사용할 수 있습니다. 임의로 만든 값이나 다른 실행의 값은 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-bytes" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
