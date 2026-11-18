import { sourceValid } from '../runner/protocol.js';
export const HANDOFF_KEY = 'rs-decimal:playground:handoff:v1';
export const SHARE_BYTES = 4096;
const MAX_FRAGMENT = 32768;
const commitValid = value => typeof value === 'string' && /^[a-f0-9]{40}$/.test(value);
export function encodeTransfer(source, commit) {
  if (!sourceValid(source) || !commitValid(commit)) throw Error('Invalid transfer');
  return JSON.stringify({ schema: 1, commit, source });
}
export function decodeTransfer(text, commit) {
  if (typeof text !== 'string' || text.length > 65536 * 6 + 128) throw Error('Invalid transfer');
  const value = JSON.parse(text);
  if (!value || Object.keys(value).sort().join(',') !== 'commit,schema,source' || value.schema !== 1 || !commitValid(commit) || value.commit !== commit || !sourceValid(value.source)) throw Error('Invalid transfer');
  return value.source;
}
export function encodeShare(source, commit) {
  if (!sourceValid(source) || new TextEncoder().encode(source).length > SHARE_BYTES) throw Error('Share too large');
  const bytes = new TextEncoder().encode(encodeTransfer(source, commit));
  const encoded = btoa(String.fromCharCode(...bytes)).replaceAll('+', '-').replaceAll('/', '_').replace(/=+$/, '');
  const fragment = '#share=' + encoded;
  if (fragment.length > MAX_FRAGMENT) throw Error('Share too large');
  return fragment;
}
export function decodeShare(fragment, commit) {
  if (typeof fragment !== 'string' || fragment.length > MAX_FRAGMENT || !/^#share=[A-Za-z0-9_-]+$/.test(fragment)) throw Error('Invalid share');
  const encoded = fragment.slice(7);
  const binary = atob(encoded.replaceAll('-', '+').replaceAll('_', '/'));
  const source = decodeTransfer(new TextDecoder('utf-8', { fatal: true }).decode(Uint8Array.from(binary, char => char.charCodeAt(0))), commit);
  if (encodeShare(source, commit) !== fragment) throw Error('Invalid share');
  return source;
}
