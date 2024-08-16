// Constructs an externally resolvable `Promise`.
//
// In some cases it is more ergonomic to resolve a Promise externally. Meaning, outside the
// executor callback. This procedure provides support in this case.
//
// ```ts
// const [result, resolve, reject] = promise<T>();
// ```
export function promise<T>(): [Promise<T>, (value: T) => void, (error: any) => void] {
  let resolve0: ((value: T) => void) | null = null;
  let reject0: ((error: any) => void) | null = null;
  const incomplete: Promise<T> = new Promise((resolve1, reject1) => {
    // This lambda is guaranteed to be invoked by the constructor of Promise.
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/Promise
    resolve0 = resolve1;
    reject0 = reject1;
  });
  return [incomplete, resolve0!, reject0!];
}

// Constructs an externally resolvable `Promise` which completes without a value.
// 
// This procedure is useful for representing the completion of a side-effect which is
// either successful or not-successful but has no return value.
//
// ```ts
// const [result, complete] = fallible();
// write(value, complete);
// return result;
// ```
//
// If `complete` is passed null, the promise will resolve with no value.
// If `complete` is passed a non-null value, the promise will reject with the value.
export function fallible(): [Promise<void>, (error: any) => void] {
  const [result, resolve, reject] = promise<void>();
  const handler = (error: any) => {
    if (error) {
      reject(error);
    } else {
      resolve();
    }
  }
  return [result, handler];
}

