---
locale: ko
source_revision: 4a909248a2e14bd655a50dd72e826f6d87a3347b735e76ddfe31ae9a4d4b4671
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

## 브라우저 제한

SQL 텍스트/바이트 입력과 오류 메시지는 UTF-8 8192바이트까지입니다. SQL 텍스트는 바깥 따옴표 제거 후 [리소스 검사](../resource-policy)를 받으므로 따옴표로 지수 제한을 우회할 수 없습니다. 빌림 호출은 입력을 유지하며 소비되거나 해제된 핸들은 재사용할 수 없습니다. Rust 트레이트/가져오기 예제는 네이티브 테스트에서 실행하며 브라우저 메서드 지원을 뜻하지 않습니다.

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
