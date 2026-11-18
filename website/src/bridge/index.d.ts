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
  precision(): number;
  sign(): number;
  is_zero(): boolean;
  is_not_zero(): boolean;
  is_positive(): boolean;
  is_negative(): boolean;
  big_int(): bigint;
  cmp(other: Decimal): number;
  equal(other: Decimal): boolean;
  not_equal(other: Decimal): boolean;
  gt(other: Decimal): boolean;
  gte(other: Decimal): boolean;
  lt(other: Decimal): boolean;
  lte(other: Decimal): boolean;
  abs(): Decimal;
  neg(): Decimal;
  max(other: Decimal): Decimal;
  min(other: Decimal): Decimal;
  add(other: Decimal): Decimal;
  sub(other: Decimal): Decimal;
  safe_add(other: Decimal): Decimal;
  safe_sub(other: Decimal): Decimal;
  mul_down(other: Decimal): Decimal;
  quo_down(other: Decimal): Decimal;
  add_raw(value: bigint): Decimal;
  sub_raw(value: bigint): Decimal;
  mul(other: Decimal, mode: Rounding): Decimal;
  quo(other: Decimal, mode: Rounding): Decimal;
  mul_with_prec(other: Decimal, prec: number, mode: Rounding): Decimal;
  quo_with_prec(other: Decimal, prec: number, mode: Rounding): Decimal;
  int_part(): bigint;
  is_integer(): boolean;
  has_fraction(): boolean;
  /** Panics on negative input; start a fresh run after a Wasm trap. */
  must_non_negative(): Decimal;
  power(power: bigint): Decimal;
  quo_rem(other: Decimal): [Decimal, Decimal];
  modulo(other: Decimal): Decimal;
  /** Rust r#mod; same signed remainder as modulo. */
  mod(other: Decimal): Decimal;
  remainder(): [bigint, bigint];
  /** Consumes this wrapper, discarding scale and initialized state. */
  into_coefficient(): bigint;
  /** Returns an original wrapper; disposing or moving it affects that alias. */
  max_ref(other: Decimal): Decimal;
  min_ref(other: Decimal): Decimal;
  /** Consumes this wrapper; rejects self-aliasing before the move. */
  add_owned(other: Decimal): Decimal;
  sub_owned(other: Decimal): Decimal;
  mul_exact(other: Decimal): Decimal;
  ceil(): Decimal;
  floor(): Decimal;
  round(): Decimal;
  truncate(): Decimal;
  strip_trailing_zeros(): Decimal;
  ceil_with_prec(prec: number): Decimal;
  floor_with_prec(prec: number): Decimal;
  round_with_prec(prec: number): Decimal;
  truncate_with_prec(prec: number): Decimal;
  rescale_down(prec: number): Decimal;
  shift(places: number): Decimal;
  significant_figures(figures: number, mode: Rounding): Decimal;
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
  readonly COMPATIBILITY_REVISION: string;
  Decimal: {
    "new": (value: bigint) => Decimal;
    new_from_string(text: string): Decimal;
    /** Panics on invalid Rust input; in Wasm this traps and ends the run. */
    must_from_string(text: string): Decimal;
    try_from_coefficient(coefficient: bigint, scale: number): Decimal;
    new_from_int(value: number): Decimal;
    new_from_decimal(value: Decimal): Decimal;
    new_with_prec(value: bigint, prec: number): Decimal;
    new_from_int64(value: bigint, prec: number): Decimal;
    new_from_uint64(value: bigint, prec: number): Decimal;
    new_from_big_int(value: bigint): Decimal;
    new_from_big_int_with_prec(value: bigint, prec: number): Decimal;
    new_with_append_prec(value: bigint, prec: number): Decimal;
    new_from_uint_with_append_prec(value: bigint, prec: number): Decimal;
    default(): Decimal;
  };
  NullDecimal: { "new": (decimal: Decimal) => NullDecimal; default(): NullDecimal };
  ByteBuffer: { "new": (bytes: Uint8Array) => ByteBuffer };
  RoundingMode: typeof RoundingMode & { from_raw(value: number): Rounding };
  snapshot(value: Decimal|NullDecimal): Readonly<Record<string, unknown>>;
  zero(): Decimal;
  one(): Decimal;
  ten(): Decimal;
  hundred(): Decimal;
  min(a: Decimal, b: Decimal): Decimal;
  max(a: Decimal, b: Decimal): Decimal;
  between(v: Decimal, lower: Decimal, upper: Decimal): boolean;
  metadata(): Record<string, unknown>;
}
export function createBridge(raw: unknown): Bridge;
