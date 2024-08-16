export function promise<T>(): [Promise<T>, (value: T) => void, (error: Error) => void] {
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

export function fallible(): [Promise<null>, (error: any) => void] {
  const [result, resolve, reject] = promise<null>();
  const handler = (error: any) => {
    if (error) {
      reject(error);
    } else {
      resolve(null);
    }
  }
  return [result, handler];
}
