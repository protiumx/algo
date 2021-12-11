package solutions

func MinPathSum(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	costs := make([][]int, m)
	for i := range costs {
		costs[i] = make([]int, n)
	}
	costs[0][0] = grid[0][0]

	for i := 1; i < m; i++ {
		costs[i][0] = grid[i][0] + costs[i-1][0]
	}
	for i := 1; i < n; i++ {
		costs[0][i] = grid[0][i] + costs[0][i-1]
	}
	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			costs[i][j] = grid[i][j] + min(costs[i-1][j], costs[i][j-1])
		}
	}
	return costs[m-1][n-1]
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
