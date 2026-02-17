import { xchacha20poly1305 } from "@noble/ciphers/chacha.js";
import { bytesToUtf8, utf8ToBytes, randomBytes } from "@noble/ciphers/utils.js";

export function decryptMessage(
  key: Uint8Array,
  data: Uint8Array,
  nonce: Uint8Array,
) {
  const cipher = xchacha20poly1305(key, nonce);
  const output = cipher.decrypt(data);
  return output;
}

export function encryptMessage(key: Uint8Array, data: Uint8Array) {
  const nonce = randomBytes(24);
  const cipher = xchacha20poly1305(key, nonce);
  const message = cipher.encrypt(data);
  return {
    message,
    nonce,
  };
}

export function decryptPayload<T>(
  key: Uint8Array,
  data: Uint8Array,
  nonce: Uint8Array,
): T {
  const output = decryptMessage(key, data, nonce);
  const payload = bytesToUtf8(output);
  return JSON.parse(payload);
}

export function encryptPayload<T>(payload: T, key: Uint8Array) {
  const encoded = JSON.stringify(payload);
  const encodedBytes = utf8ToBytes(encoded);

  const nonce = randomBytes(24);
  const cipher = xchacha20poly1305(key, nonce);
  const message = cipher.encrypt(encodedBytes);

  return {
    nonce,
    message,
  };
}
