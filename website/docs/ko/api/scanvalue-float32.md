---
locale: ko
source_revision: 15c94f02827b09a573140cb09fd7743d1624bd74d93875d8d0450c89214f2677
translation_provenance: en-US-korean-reviewed
api_id: "ScanValue::Float32"
---
# ScanValue::Float32

SQL 입력 변형 Float32을 선택합니다.

```rust
Float32 (f32)
```

## 매개변수, 반환값, 일반적인 사용법

binary32를 담습니다. 브라우저는 number를 받고 Rust는 scan 전에 f32로 좁힙니다. NaN, 무한대, 축소 오버플로는 Unmarshal입니다. 변형은 scan에 빌려 전달하며 한 브라우저 실행 안에서 재사용할 수 있습니다. 임의로 만든 값이나 다른 실행의 값은 거부합니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-scanvalue-float32" locale="ko" />

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
