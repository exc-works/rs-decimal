---
locale: ko
source_revision: ab8ea6bf03391270a33235fc649739ef25b1cdab822e201005d40879c5be9299
translation_provenance: en-US-korean-reviewed
api_id: "Decimal::value"
---
# Decimal::value

정규 SQL 값을 얻습니다.

```rust
fn value (& self) -> String
```

## 매개변수, 반환값, 일반적인 사용법

매개변수 없음. String/string을 반환합니다. 미초기화 Decimal도 NULL이 아닌 "0"을 반환하므로 SQL NULL에는 NullDecimal을 사용하세요. 소수 자릿수나 초기화를 바꾸지 않고 소수 부분의 후행 0을 제거합니다. 반환 텍스트는 스칼라 값이며 따옴표로 감싼 SQL 쿼리 조각이 아닙니다.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="scan-errors-decimal-value" locale="ko" />

## 브라우저 제한

SQL 텍스트/바이트 입력과 오류 메시지는 UTF-8 8192바이트까지입니다. SQL 텍스트는 바깥 따옴표 제거 후 [리소스 검사](../resource-policy)를 받으므로 따옴표로 지수 제한을 우회할 수 없습니다. 빌림 호출은 입력을 유지하며 소비되거나 해제된 핸들은 재사용할 수 없습니다. Rust 트레이트/가져오기 예제는 네이티브 테스트에서 실행하며 브라우저 메서드 지원을 뜻하지 않습니다.

## 관련 메서드

[scan](./decimal-scan) · [ScanValue](./scanvalue) · [DecimalError](./decimalerror)
