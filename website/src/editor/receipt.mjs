// Only the comments and receipt labels change between locales.
const translations = {
  'en-US': [
    'Two notebooks at 4.50 each. Change the quantity and run again.',
    'Multiply the unit price by the quantity, then add delivery.',
    'The customer pays with 20.00. Subtract the total to find the change.',
    'Notebooks', 'Delivery', 'Amount due', 'Change from 20.00',
  ],
  'zh-Hans': [
    '两本笔记本，每本 4.50 元。试着改数量，再运行一次。',
    '单价乘以数量得到商品小计，再加上运费。',
    '顾客付了 20.00 元。用实付金额减去应付金额，算出找零。',
    '商品小计', '运费', '应付金额', '付 20.00 元找零',
  ],
  'zh-Hant': [
    '兩本筆記本，每本 4.50 元。試著改數量，再執行一次。',
    '單價乘以數量得到商品小計，再加上運費。',
    '顧客付了 20.00 元。用實付金額減去應付金額，算出找零。',
    '商品小計', '運費', '應付金額', '付 20.00 元找零',
  ],
  fr: [
    'Deux carnets à 4,50 pièce. Changez la quantité et relancez le calcul.',
    'Multipliez le prix unitaire par la quantité, puis ajoutez la livraison.',
    'La cliente donne 20,00. Soustrayez le total pour calculer la monnaie à rendre.',
    'Carnets', 'Livraison', 'Montant à payer', 'Monnaie sur 20,00',
  ],
  de: [
    'Zwei Hefte zu je 4,50. Ändern Sie die Menge und starten Sie erneut.',
    'Multiplizieren Sie den Stückpreis mit der Menge und addieren Sie den Versand.',
    'Die Kundin zahlt mit 20,00. Ziehen Sie den Gesamtbetrag ab, um das Rückgeld zu berechnen.',
    'Hefte', 'Versand', 'Zu zahlen', 'Rückgeld auf 20,00',
  ],
  ja: [
    '単価 4.50 のノートを 2 冊買います。数量を変えて、もう一度実行してみましょう。',
    '単価に数量を掛けて小計を求め、送料を足します。',
    'お客様から預かった 20.00 から支払額を引くと、お釣りが求まります。',
    '商品小計', '送料', '支払額', '20.00 に対するお釣り',
  ],
  ko: [
    '단가 4.50인 공책 두 권입니다. 수량을 바꾸고 다시 실행해 보세요.',
    '단가에 수량을 곱해 소계를 구하고 배송비를 더합니다.',
    '고객이 낸 20.00에서 결제 금액을 빼면 거스름돈이 됩니다.',
    '상품 소계', '배송비', '결제 금액', '20.00을 낸 경우 거스름돈',
  ],
};
translations['en-GB'] = translations['en-US'];

export function receiptSource(locale) {
  const [order, calculate, payment, subtotal, delivery, total, change] = translations[locale];
  return `// ${order}
const price = Decimal.new_from_string("4.50");
const quantity = Decimal.new(2n);

// ${calculate}
const subtotal = price.mul_exact(quantity);
const delivery = Decimal.new_from_string("1.25");
const total = subtotal.add(delivery);
show("${subtotal}: " + subtotal.string_with_trailing_zeros());
show("${delivery}: " + delivery.string_with_trailing_zeros());
show("${total}: " + total.string_with_trailing_zeros());

// ${payment}
const paid = Decimal.new_from_string("20.00");
show("${change}: " + paid.sub(total).string_with_trailing_zeros());
`;
}

// Recognize an untouched starter saved by an earlier visit. Custom drafts stay verbatim.
const stockSources = new Set(Object.keys(translations).map(receiptSource));
stockSources.add(`const price = Decimal.new_from_string("12.350");
const total = price.mul_exact(Decimal.new(3n));
show(total);
assert(total.string() === "37.05");
show(total.rescale(1, RoundingMode.HalfEven));
`);

export function isStockSource(source) {
  return stockSources.has(source);
}
