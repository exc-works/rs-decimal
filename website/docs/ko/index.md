---
locale: ko
source_revision: 92be186c5740ac6d22b3ca742b8ced1f5ab7822e9ab835f05db11a750bdadf79
aside: false
---
# rs-decimal

rs-decimal은 Rust에서 십진수를 다루는 라이브러리입니다. 가격을 더하거나 단가에 수량을 곱하고, 영수증에 금액을 소수점 아래 몇 자리까지 표시할지 정할 수 있습니다.

## 영수증의 금액은 얼마일까요?

단가가 4.50인 공책 두 권에 배송비 1.25를 더하면 결제 금액은 10.25입니다. 주문서의 단가를 읽는 일부터 영수증에 금액을 표시하는 일까지 Decimal로 처리할 수 있습니다.

<div class="receipt-equation">4.50 × 2 + 1.25 = <strong>10.25</strong></div>

<div class="ref-actions">

[첫 주문 금액 계산하기](/ko/guide)
[브라우저에서 수량 바꿔 보기](/ko/playground)

</div>
