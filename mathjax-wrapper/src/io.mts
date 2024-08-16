import assert from "node:assert";
import { once } from "node:events";
import { Readable, Writable } from "node:stream";
import { fallible } from "./util.mjs";

// Asynchronously consumes `n` bytes from the given stream.
// - If the stream is closed, the promise will reject.
// - If the stream emits an error, the promise will reject.
// - If `n` bytes are consumed, a buffer containing them is returned.
export async function readn(stream: Readable, n: number): Promise<Buffer> {
  assert(Number.isInteger(n), "n must be n an integer");
  assert(n >= 0, "n must be non-negative");
  if (n == 0) return Buffer.alloc(0); // A `readable` event will never be emitted.
  let buffer: Buffer | null = null;
  while (buffer === null) {
    await once(stream, "readable");
    buffer = stream.read(n);
  }
  assert(buffer.byteLength === n, "expected n=" + n + " bytes but got" + buffer.byteLength);
  return buffer;
}


// Writes the entirety of the given byte buffer `buf` to the output stream `destin`.
// The output stream `destin` **will not** be closed. Therefore, it is possible to
// invoke this procedure multiple times on the same output stream. 
//
// Returns a `Promise` which resolves when the buffer has been written to `destin` or
// rejects if an error occurs while writing to `destin`.
export async function write(buf: Buffer, destin: Writable): Promise<null> {
  const [result, handler] = fallible();
  if (!destin.write(buf, handler)) {
    await once(destin, 'drain');
  }
  return result;
}

export function end(stream: Writable): Promise<void> {
  return new Promise(resolve => {
    stream.end(() => resolve());
  })
}
