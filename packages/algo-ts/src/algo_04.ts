export function MinPathSum(grid: number[][]): number {
  const m = grid.length;
  const n = grid[0].length;
  const costs = Array.from(Array(m), () => new Array(n));

  costs[0][0] = grid[0][0];
  for (let i = 1; i < m; i++) {
    costs[i][0] = grid[i][0] + costs[i - 1][0];
  }
  for (let j = 1; j < n; j++) {
    costs[0][j] = grid[0][j] + costs[0][j - 1];
  }
  for (let i = 1; i < m; i++) {
    for (let j = 1; j < n; j++) {
      costs[i][j] = grid[i][j] + Math.min(costs[i - 1][j], costs[i][j - 1]);
    }
  }
  return costs[m - 1][n - 1];
}

