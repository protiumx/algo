export function maxSubstr(input: string): number {
  let max = 0;
  let start = 0;
  const indexes = Array(128).fill(-1);
  for (let i = 0; i < input.length; i++) {
    if (indexes[input.charCodeAt(i)] > start) {
      start = indexes[input.charCodeAt(i)] + 1;
    }
    indexes[input.charCodeAt(i)] = i;
    max = Math.max(max, i - start + 1);
  }

  return max;
}
