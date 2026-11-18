export type Rounding = 0 | 1 | 2 | 3 | 4 | 5 | 6;
export const RoundingMode: Readonly<Record<'Down'|'Up'|'Ceiling'|'HalfUp'|'HalfDown'|'HalfEven'|'Unnecessary', Rounding>>;
export class BridgeError extends Error { category: string; kinds: string[]; }
export interface Decimal {
  clone(): Decimal;
  coefficient(): bigint;
  scale(): number;
  is_nil(): boolean;
  string(): string;
  string_with_trailing_zeros(): string;
  mul_exact(other: Decimal): Decimal;
  rescale(prec: number, mode: Rounding): Decimal;
  /** Consumes this value, even if the Rust operation returns an error. */
  into_rescaled(prec: number, mode: Rounding): Decimal;
  free(): void;
}
export interface NullDecimal {
  valid: boolean;
  /** Explicit copy, not a live reference to the nullable field. */
  get_decimal_clone(): Decimal;
  /** Consumes the argument and preserves this nullable's valid flag. */
  set_decimal(value: Decimal): void;
  free(): void;
}
export interface ByteBuffer { to_uint8_array(): Uint8Array; free(): void; }
export interface Bridge {
  Decimal: { "new": (value: bigint) => Decimal; new_from_string(text: string): Decimal; default(): Decimal };
  NullDecimal: { "new": (decimal: Decimal) => NullDecimal; default(): NullDecimal };
  ByteBuffer: { "new": (bytes: Uint8Array) => ByteBuffer };
  RoundingMode: typeof RoundingMode;
  snapshot(value: Decimal|NullDecimal): Readonly<Record<string, unknown>>;
  metadata(): Record<string, unknown>;
}
export function createBridge(raw: unknown): Bridge;
