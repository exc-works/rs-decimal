// Rust-backed methods retain Rust names and order; no arithmetic lives here.
export class BridgeError extends Error {
  constructor(category, message) {
    super(message);
    this.name = 'BridgeError';
    this.category = category;
    this.kinds = [];
  }
}

function requireType(condition, message) {
  if (!condition) throw new BridgeError('parameter', message);
}

function resource(condition, message) {
  if (!condition) throw new BridgeError('resource', message);
}

function integer(value, name, min = -2147483648, max = 2147483647) {
  requireType(typeof value === 'number' && Number.isInteger(value) && value >= min && value <= max,
    `${name} must be an integer between ${min} and ${max}`);
  return value;
}

export const RoundingMode = Object.freeze({
  Down: 0, Up: 1, Ceiling: 2, HalfUp: 3, HalfDown: 4, HalfEven: 5, Unnecessary: 6,
});

const BIGINT_LIMIT = 10n ** 4096n;
function bigintText(value, unsigned = false) {
  requireType(typeof value === 'bigint', 'value must be bigint');
  const min = unsigned ? 0n : -(1n << 63n), max = unsigned ? 1n << 64n : 1n << 63n;
  requireType(value >= min && value < max, unsigned ? 'value must fit u64' : 'value must fit i64');
  return value.toString();
}
function bigText(value) {
  requireType(typeof value === 'bigint', 'value must be bigint');
  resource(value > -BIGINT_LIMIT && value < BIGINT_LIMIT, 'playground coefficient exceeds 4096 digits');
  return value.toString();
}

// A factory keeps wrapper identities tied to one Wasm instance/run.
export function createBridge(raw) {
  const key = Symbol('bridge construction');
  const slots = new WeakMap();
  function wrap(Type, value) { return new Type(key, value); }
  function borrow(value, Type) {
    requireType(value instanceof Type, `expected ${Type.name} from this run`);
    const handle = slots.get(value);
    if (!handle) throw new BridgeError('moved', 'value has been consumed or disposed');
    return handle;
  }
  function take(value, Type) {
    const handle = borrow(value, Type);
    slots.delete(value);
    return handle;
  }
  class Handle {
    constructor(token, handle) {
      requireType(token === key, 'use a named constructor');
      slots.set(this, handle);
    }
    free() {
      const handle = slots.get(this);
      if (handle) { slots.delete(this); handle.free(); }
    }
  }
  class Decimal extends Handle {
    static new(value) {
      requireType(typeof value === 'bigint', 'value must be bigint (for example 3n)');
      requireType(value >= -(1n << 63n) && value < (1n << 63n), 'value must fit i64');
      return wrap(Decimal, raw.Decimal.new_i64_text(value.toString()));
    }
    static new_from_string(text) {
      requireType(typeof text === 'string', 'text must be a string');
      resource(text.length <= 8192, 'playground input exceeds the input budget');
      return wrap(Decimal, raw.Decimal.new_from_string(text));
    }
    static must_from_string(text) {
      requireType(typeof text === 'string', 'text must be a string');
      resource(text.length <= 8192, 'playground input exceeds the input budget');
      return wrap(Decimal, raw.Decimal.must_from_string(text));
    }
    static new_from_int(value) { return wrap(Decimal, raw.Decimal.new_from_int(integer(value, 'value'))); }
    static new_from_decimal(value) { return wrap(Decimal, raw.Decimal.new_from_decimal(borrow(value, Decimal))); }
    static new_with_prec(value, prec) { return wrap(Decimal, raw.Decimal.new_with_prec(bigintText(value), integer(prec, 'prec'))); }
    static new_from_int64(value, prec) { return wrap(Decimal, raw.Decimal.new_from_int64(bigintText(value), integer(prec, 'prec'))); }
    static new_from_uint64(value, prec) { return wrap(Decimal, raw.Decimal.new_from_uint64(bigintText(value, true), integer(prec, 'prec'))); }
    static new_from_big_int(value) { return wrap(Decimal, raw.Decimal.new_from_big_int(bigText(value))); }
    static new_from_big_int_with_prec(value, prec) { return wrap(Decimal, raw.Decimal.new_from_big_int_with_prec(bigText(value), integer(prec, 'prec'))); }
    static new_with_append_prec(value, prec) { return wrap(Decimal, raw.Decimal.new_with_append_prec(bigintText(value), integer(prec, 'prec'))); }
    static new_from_uint_with_append_prec(value, prec) { return wrap(Decimal, raw.Decimal.new_from_uint_with_append_prec(bigintText(value, true), integer(prec, 'prec'))); }
    static default() { return wrap(Decimal, raw.Decimal.default()); }
    clone() { return wrap(Decimal, borrow(this, Decimal).clone()); }
    coefficient() { return BigInt(borrow(this, Decimal).coefficient_text()); }
    scale() { return borrow(this, Decimal).scale(); }
    is_nil() { return borrow(this, Decimal).is_nil(); }
    string() { return borrow(this, Decimal).string(); }
    string_with_trailing_zeros() { return borrow(this, Decimal).string_with_trailing_zeros(); }
    precision() { return borrow(this, Decimal).precision(); }
    sign() { return borrow(this, Decimal).sign(); }
    is_zero() { return borrow(this, Decimal).is_zero(); }
    is_not_zero() { return borrow(this, Decimal).is_not_zero(); }
    is_positive() { return borrow(this, Decimal).is_positive(); }
    is_negative() { return borrow(this, Decimal).is_negative(); }
    big_int() { return BigInt(borrow(this, Decimal).big_int_text()); }
    cmp(other) { return borrow(this, Decimal).cmp(borrow(other, Decimal)); }
    equal(other) { return borrow(this, Decimal).equal(borrow(other, Decimal)); }
    not_equal(other) { return borrow(this, Decimal).not_equal(borrow(other, Decimal)); }
    gt(other) { return borrow(this, Decimal).gt(borrow(other, Decimal)); }
    gte(other) { return borrow(this, Decimal).gte(borrow(other, Decimal)); }
    lt(other) { return borrow(this, Decimal).lt(borrow(other, Decimal)); }
    lte(other) { return borrow(this, Decimal).lte(borrow(other, Decimal)); }
    abs() { return wrap(Decimal, borrow(this, Decimal).abs()); }
    neg() { return wrap(Decimal, borrow(this, Decimal).neg()); }
    max(other) { return wrap(Decimal, borrow(this, Decimal).max(borrow(other, Decimal))); }
    min(other) { return wrap(Decimal, borrow(this, Decimal).min(borrow(other, Decimal))); }
    mul_exact(other) {
      return wrap(Decimal, borrow(this, Decimal).mul_exact(borrow(other, Decimal)));
    }
    ceil() { return wrap(Decimal, borrow(this, Decimal).ceil()); }
    floor() { return wrap(Decimal, borrow(this, Decimal).floor()); }
    round() { return wrap(Decimal, borrow(this, Decimal).round()); }
    truncate() { return wrap(Decimal, borrow(this, Decimal).truncate()); }
    strip_trailing_zeros() { return wrap(Decimal, borrow(this, Decimal).strip_trailing_zeros()); }
    ceil_with_prec(prec) { return wrap(Decimal, borrow(this, Decimal).ceil_with_prec(integer(prec, "prec"))); }
    floor_with_prec(prec) { return wrap(Decimal, borrow(this, Decimal).floor_with_prec(integer(prec, "prec"))); }
    round_with_prec(prec) { return wrap(Decimal, borrow(this, Decimal).round_with_prec(integer(prec, "prec"))); }
    truncate_with_prec(prec) { return wrap(Decimal, borrow(this, Decimal).truncate_with_prec(integer(prec, "prec"))); }
    rescale_down(prec) { return wrap(Decimal, borrow(this, Decimal).rescale_down(integer(prec, "prec"))); }
    shift(places) { return wrap(Decimal, borrow(this, Decimal).shift(integer(places, 'places'))); }
    significant_figures(figures, mode) { return wrap(Decimal, borrow(this, Decimal).significant_figures(integer(figures, 'figures'), integer(mode, 'mode'))); }
    rescale(prec, mode) {
      integer(prec, 'prec'); integer(mode, 'mode');
      return wrap(Decimal, borrow(this, Decimal).rescale(prec, mode));
    }
    into_rescaled(prec, mode) {
      // JS shape/range rejection happens before a Rust call/move. Library errors
      // after the call consume self exactly as the Rust signature specifies.
      integer(prec, 'prec'); integer(mode, 'mode');
      return wrap(Decimal, take(this, Decimal).into_rescaled(prec, mode));
    }
  }
  class NullDecimal extends Handle {
    static new(decimal) { return wrap(NullDecimal, raw.NullDecimal.new(take(decimal, Decimal))); }
    static default() { return wrap(NullDecimal, raw.NullDecimal.default()); }
    get valid() { return borrow(this, NullDecimal).valid; }
    set valid(value) {
      requireType(typeof value === 'boolean', 'valid must be boolean');
      borrow(this, NullDecimal).valid = value;
    }
    get_decimal_clone() { return wrap(Decimal, borrow(this, NullDecimal).get_decimal_clone()); }
    set_decimal(value) {
      const target = borrow(this, NullDecimal);
      target.set_decimal(take(value, Decimal));
    }
  }
  class ByteBuffer extends Handle {
    static new(bytes) {
      requireType(bytes instanceof Uint8Array, 'bytes must be Uint8Array');
      resource(bytes.byteLength <= 65536, 'playground buffer exceeds 65536 bytes');
      return wrap(ByteBuffer, raw.ByteBuffer.new(bytes));
    }
    to_uint8_array() { return borrow(this, ByteBuffer).to_uint8_array(); }
  }
  function snapshot(value) {
    if (value instanceof Decimal) {
      const handle = borrow(value, Decimal);
      return Object.freeze({ ...JSON.parse(handle.snapshot()), string: handle.string(), string_with_trailing_zeros: handle.string_with_trailing_zeros() });
    }
    if (value instanceof NullDecimal) {
      const data = JSON.parse(borrow(value, NullDecimal).snapshot());
      return Object.freeze({ ...data, decimal: Object.freeze(data.decimal) });
    }
    throw new BridgeError('parameter', 'snapshot expects a Decimal or NullDecimal from this run');
  }
  const rounding = Object.freeze({ ...RoundingMode,
    from_raw: value => raw.rounding_mode_from_raw(integer(value, 'value')),
  });
  const helpers = {
    zero: () => wrap(Decimal, raw.zero()),
    one: () => wrap(Decimal, raw.one()),
    ten: () => wrap(Decimal, raw.ten()),
    hundred: () => wrap(Decimal, raw.hundred()),
    min: (a, b) => wrap(Decimal, raw.min(borrow(a, Decimal), borrow(b, Decimal))),
    max: (a, b) => wrap(Decimal, raw.max(borrow(a, Decimal), borrow(b, Decimal))),
    between: (v, lower, upper) => raw.between(borrow(v, Decimal), borrow(lower, Decimal), borrow(upper, Decimal)),
  };
  return Object.freeze({ Decimal, NullDecimal, ByteBuffer, RoundingMode: rounding, snapshot, ...helpers, get COMPATIBILITY_REVISION() { return raw.compatibility_revision(); }, metadata: () => JSON.parse(raw.metadata()) });
}
