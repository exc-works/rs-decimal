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
    static default() { return wrap(Decimal, raw.Decimal.default()); }
    clone() { return wrap(Decimal, borrow(this, Decimal).clone()); }
    coefficient() { return BigInt(borrow(this, Decimal).coefficient_text()); }
    scale() { return borrow(this, Decimal).scale(); }
    is_nil() { return borrow(this, Decimal).is_nil(); }
    string() { return borrow(this, Decimal).string(); }
    string_with_trailing_zeros() { return borrow(this, Decimal).string_with_trailing_zeros(); }
    mul_exact(other) {
      return wrap(Decimal, borrow(this, Decimal).mul_exact(borrow(other, Decimal)));
    }
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
  return Object.freeze({ Decimal, NullDecimal, ByteBuffer, RoundingMode, snapshot, metadata: () => JSON.parse(raw.metadata()) });
}
