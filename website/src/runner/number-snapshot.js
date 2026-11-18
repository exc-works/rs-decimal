// Preserve values that JSON would turn into null or positive zero.
export function numberSnapshot(value) {
  if (Number.isFinite(value) && !Object.is(value, -0)) return value;
  const bytes = new DataView(new ArrayBuffer(8));
  bytes.setFloat64(0, value, false);
  return {
    type: 'float64',
    value: Object.is(value, -0) ? '-0' : String(value),
    bits: bytes.getBigUint64(0, false).toString(16).padStart(16, '0'),
  };
}
