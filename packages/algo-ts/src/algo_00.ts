export function mostFrequent(arr: number[]): number {
  let ret = -1;
  let maxCount = -1;
  const counter = new Map<number, number>();
  for (const n of arr) {
    let count = 1;
    if (!counter.has(n)) {
      counter.set(n, count);
    } else {
      count = counter.get(n)!;
      count++;
    }
    if (count > maxCount) {
      maxCount = count;
      ret = n;
    }
  }
  return ret;
}
